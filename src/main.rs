use clap::Parser;
use humantime::parse_duration;
#[cfg(unix)]
use rustix::process;
use std::io;
use std::process::ExitStatus;
use std::str::FromStr;
use std::time::Duration;
use thiserror::Error;
use tokio::process::{Child, Command};
use tokio::time::{sleep, timeout};
use tracing::level_filters::LevelFilter;
use tracing::{debug, info, warn};
use tracing_subscriber::fmt as tracing_fmt;

#[derive(Error, Debug)]
pub enum Error {
    #[error("command is required")]
    CommandRequired,
    #[error("failed to start command '{0}': {1}")]
    SpawnCommand(String, #[source] std::io::Error),
    #[error("failed to wait for command: {0}")]
    WaitChild(#[source] std::io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug)]
struct SignalArg {
    label: String,
    #[cfg(unix)]
    number: i32,
}

impl SignalArg {
    fn label(&self) -> &str {
        &self.label
    }

    fn parse(raw: &str) -> std::result::Result<Self, String> {
        raw.parse()
    }
}

impl FromStr for SignalArg {
    type Err = String;

    fn from_str(input: &str) -> std::result::Result<Self, Self::Err> {
        let trimmed = input.trim();
        if trimmed.is_empty() {
            return Err("signal cannot be empty".to_string());
        }

        let upper = trimmed.to_ascii_uppercase();
        let normalized = upper.strip_prefix("SIG").unwrap_or(&upper);

        #[cfg(unix)]
        {
            if let Some(parsed) = normalized.parse::<i32>().ok().and_then(|num| {
                map_signal_number(num).map(|label| SignalArg { label, number: num })
            }) {
                return Ok(parsed);
            }

            if let Some((num, label)) = map_signal_name(normalized) {
                return Ok(SignalArg {
                    label: label.to_string(),
                    number: num,
                });
            }

            Err(format!(
                "invalid signal '{input}'. Use names like TERM, INT, KILL or their numbers"
            ))
        }

        #[cfg(not(unix))]
        Ok(SignalArg {
            label: format!("SIG{}", normalized),
        })
    }
}

fn parse_duration_arg(input: &str) -> std::result::Result<Duration, String> {
    parse_duration(input).map_err(|source| {
        format!("invalid duration '{input}': {source}. Try values like 30s, 2m, 500ms, 1h")
    })
}

fn parse_kill_after_arg(input: &str) -> std::result::Result<Duration, String> {
    parse_duration(input).map_err(|source| {
        format!("invalid kill-after duration '{input}': {source}. Try values like 5s, 1m, 500ms")
    })
}

#[cfg(unix)]
fn map_signal_number(num: i32) -> Option<String> {
    match num {
        n if n == libc::SIGHUP => Some("SIGHUP".to_string()),
        n if n == libc::SIGINT => Some("SIGINT".to_string()),
        n if n == libc::SIGQUIT => Some("SIGQUIT".to_string()),
        n if n == libc::SIGKILL => Some("SIGKILL".to_string()),
        n if n == libc::SIGTERM => Some("SIGTERM".to_string()),
        n if n == libc::SIGUSR1 => Some("SIGUSR1".to_string()),
        n if n == libc::SIGUSR2 => Some("SIGUSR2".to_string()),
        n if n == libc::SIGALRM => Some("SIGALRM".to_string()),
        n if n == libc::SIGCHLD => Some("SIGCHLD".to_string()),
        _ => None,
    }
}

#[cfg(unix)]
fn map_signal_name(name: &str) -> Option<(i32, &'static str)> {
    match name {
        "HUP" => Some((libc::SIGHUP, "SIGHUP")),
        "INT" => Some((libc::SIGINT, "SIGINT")),
        "QUIT" => Some((libc::SIGQUIT, "SIGQUIT")),
        "KILL" => Some((libc::SIGKILL, "SIGKILL")),
        "TERM" => Some((libc::SIGTERM, "SIGTERM")),
        "ALRM" => Some((libc::SIGALRM, "SIGALRM")),
        "USR1" => Some((libc::SIGUSR1, "SIGUSR1")),
        "USR2" => Some((libc::SIGUSR2, "SIGUSR2")),
        "CHLD" => Some((libc::SIGCHLD, "SIGCHLD")),
        _ => None,
    }
}

fn init_tracing(verbose: bool, quiet: bool) {
    let max_level = if verbose {
        LevelFilter::DEBUG
    } else if quiet {
        LevelFilter::ERROR
    } else {
        LevelFilter::INFO
    };

    let _ = tracing_fmt()
        .with_writer(std::io::stderr)
        .with_max_level(max_level)
        .without_time()
        .try_init();
}

enum ProcessController {
    #[cfg(unix)]
    Unix,
    #[cfg(windows)]
    Windows,
    #[cfg(not(any(unix, windows)))]
    Generic,
}

impl ProcessController {
    async fn request_graceful(&self, child: &mut Child, signal: &SignalArg) -> io::Result<()> {
        match self {
            #[cfg(unix)]
            Self::Unix => {
                let pid = child
                    .id()
                    .ok_or_else(|| io::Error::other("child pid unavailable"))?
                    as i32;
                let pgid = pid;
                send_signal(pgid, signal.number, signal.label())
            }
            #[cfg(windows)]
            Self::Windows => {
                let _ = signal; // Unused on Windows (no signal support)
                if let Some(pid) = child.id() {
                    warn!(
                        pid,
                        "Graceful termination is not implemented on Windows; waiting for kill-after"
                    );
                }
                Ok(())
            }
            #[cfg(not(any(unix, windows)))]
            Self::Generic => {
                let _ = signal; // Unused on this platform
                warn!("Graceful termination not supported on this platform");
                Ok(())
            }
        }
    }

    async fn force_terminate(&self, child: &mut Child) -> io::Result<()> {
        match self {
            #[cfg(unix)]
            Self::Unix => {
                let pid = child
                    .id()
                    .ok_or_else(|| io::Error::other("child pid unavailable"))?
                    as i32;
                send_signal(pid, libc::SIGKILL, "SIGKILL")?;
                Ok(())
            }
            #[cfg(windows)]
            Self::Windows => child.kill().await,
            #[cfg(not(any(unix, windows)))]
            Self::Generic => child.kill().await,
        }
    }
}

#[cfg(unix)]
fn platform_controller() -> ProcessController {
    ProcessController::Unix
}

#[cfg(windows)]
fn platform_controller() -> ProcessController {
    ProcessController::Windows
}

#[cfg(not(any(unix, windows)))]
fn platform_controller() -> ProcessController {
    ProcessController::Generic
}

#[cfg(unix)]
fn send_signal(pgid: i32, signal: i32, label: &str) -> io::Result<()> {
    let res = unsafe { libc::killpg(pgid, signal) };
    if res == 0 {
        info!(pgid, signal = label, "Dispatched signal to process group");
        Ok(())
    } else {
        let err = io::Error::last_os_error();
        warn!(pgid, signal = label, error = %err, "Failed to dispatch signal");
        Err(err)
    }
}

#[derive(Parser, Debug)]
#[command(version = concat!("v", env!("CARGO_PKG_VERSION"), "\nCopyright (c) 2025 NuewLabs Inc"), about = env!("CARGO_PKG_DESCRIPTION"))]
struct Opt {
    /// Duration like 30s, 2m, 500ms, 1h
    #[arg(value_parser = parse_duration_arg)]
    duration: Duration,

    /// Print additional diagnostics (overrides --quiet)
    #[arg(short = 'v', long, conflicts_with = "quiet")]
    verbose: bool,

    /// Suppress timeout diagnostics (overridden by --verbose)
    #[arg(short = 'q', long, conflicts_with = "verbose")]
    quiet: bool,

    /// Signal name or number to send before force killing (Unix only)
    #[arg(short = 's', long = "signal", value_name = "SIG", default_value = "TERM", value_parser = SignalArg::parse)]
    signal: SignalArg,

    /// Grace period before force killing, e.g., 5s, 250ms
    #[arg(short = 'k', long = "kill-after", value_name = "DURATION", default_value = "5s", value_parser = parse_kill_after_arg)]
    kill_after: Duration,

    /// Command and args
    #[arg(last = true, required = true)]
    cmd_and_args: Vec<String>,
}

#[tokio::main]
async fn main() {
    match run_cli().await {
        Ok(code) => std::process::exit(code),
        Err(e) => {
            eprintln!("Error: {e}");
            std::process::exit(1);
        }
    }
}

async fn run_cli() -> Result<i32> {
    let opt = Opt::parse();
    init_tracing(opt.verbose, opt.quiet);
    let controller = platform_controller();

    let mut it = opt.cmd_and_args.into_iter();
    let cmd = it.next().ok_or(Error::CommandRequired)?;
    let args: Vec<String> = it.collect();

    run_with_timeout(
        &cmd,
        &args,
        opt.duration,
        &opt.signal,
        opt.kill_after,
        &controller,
    )
    .await
}

async fn run_with_timeout(
    cmd: &str,
    args: &[String],
    dur: Duration,
    signal: &SignalArg,
    kill_after: Duration,
    controller: &ProcessController,
) -> Result<i32> {
    let fdur = humantime::format_duration(dur).to_string();
    debug!(command = %cmd, args = ?args, "Starting command:");
    debug!(timeout = %fdur, "Timeout:");

    // Build command
    let mut command = Command::new(cmd);
    command.args(args);
    // Inherit stdio so child prints directly to terminal
    command.stdin(std::process::Stdio::inherit());
    command.stdout(std::process::Stdio::inherit());
    command.stderr(std::process::Stdio::inherit());

    // Platform-specific: create new process group so we can kill whole group
    #[cfg(unix)]
    {
        // setsid in before_exec will create a new session / process group
        unsafe {
            command.pre_exec(|| {
                // set a new session id (creates new process group)
                process::setsid()
                    .map(|_| ())
                    .map_err(|_| std::io::Error::other("setsid failed before exec"))
            });
        }
    }

    #[cfg(windows)]
    {
        // CREATE_NEW_PROCESS_GROUP = 0x00000200
        command.creation_flags(0x00000200);
    }

    let mut child = command
        .spawn()
        .map_err(|e| Error::SpawnCommand(format!("{cmd} {args:?}"), e))?;

    // Wait for either the child to exit or the timeout
    let wait_fut = child.wait();

    // Use tokio::select to await whichever finishes first
    tokio::select! {
        res = wait_fut => {
            match res {
                Ok(status) => {
                    let code = exit_status_to_code(status);
                    Ok(code)
                }
                Err(e) => {
                    Err(Error::WaitChild(e))
                }
            }
        }

        _ = sleep(dur) => {
            // timed out
            info!("Command timed out after {fdur}");
            handle_timeout(&mut child, controller, kill_after, signal).await?;
            // Return timeout exit code similar to GNU timeout
            Ok(124)
        }
    }
}

fn exit_status_to_code(status: ExitStatus) -> i32 {
    if let Some(code) = status.code() {
        return code;
    }

    // killed by signal on unix: return 128 + signal (best-effort)
    #[cfg(unix)]
    {
        use std::os::unix::process::ExitStatusExt;
        if let Some(sig) = status.signal() {
            return 128 + sig;
        }
    }

    1
}

async fn handle_timeout(
    child: &mut Child,
    controller: &ProcessController,
    kill_after: Duration,
    signal: &SignalArg,
) -> Result<()> {
    if let Err(err) = controller.request_graceful(child, signal).await {
        warn!(error = %err, signal = signal.label(), "Failed to initiate graceful termination");
    }

    if kill_after.is_zero() {
        warn!("kill-after duration is 0; forcing termination immediately");
        if let Err(err) = controller.force_terminate(child).await {
            warn!(error = %err, "Failed to force terminate child");
        }
        let _ = child.wait().await;
        return Ok(());
    }

    match timeout(kill_after, child.wait()).await {
        Ok(Ok(status)) => {
            info!(?status, "Process exited within grace period");
            Ok(())
        }
        Ok(Err(err)) => {
            warn!(error = %err, "Error while waiting for child during grace period");
            if let Err(force_err) = controller.force_terminate(child).await {
                warn!(error = %force_err, "Failed to force terminate child");
            }
            let _ = child.wait().await;
            Ok(())
        }
        Err(_) => {
            warn!(
                kill_after = %humantime::format_duration(kill_after),
                "Grace period elapsed; forcing termination"
            );
            if let Err(force_err) = controller.force_terminate(child).await {
                warn!(error = %force_err, "Failed to force terminate child");
            }
            let _ = child.wait().await;
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{exit_status_to_code, parse_duration_arg};

    fn status_from_code(code: i32) -> std::process::ExitStatus {
        #[cfg(unix)]
        {
            use std::os::unix::process::ExitStatusExt;
            std::process::ExitStatus::from_raw(code << 8)
        }

        #[cfg(windows)]
        {
            use std::os::windows::process::ExitStatusExt;
            std::process::ExitStatus::from_raw(code as u32)
        }

        #[cfg(not(any(unix, windows)))]
        {
            let _ = code;
            panic!("status_from_code not supported on this platform");
        }
    }

    #[cfg(unix)]
    fn status_from_signal(sig: i32) -> std::process::ExitStatus {
        use std::os::unix::process::ExitStatusExt;
        std::process::ExitStatus::from_raw(sig)
    }

    #[test]
    fn exit_status_preserves_code() {
        let status = status_from_code(42);
        assert_eq!(exit_status_to_code(status), 42);
    }

    #[cfg(unix)]
    #[test]
    fn exit_status_signal_maps_to_128_plus_sig() {
        let status = status_from_signal(9);
        assert_eq!(exit_status_to_code(status), 137);
    }

    #[test]
    fn parse_duration_error_is_human_readable() {
        let message = parse_duration_arg("3x").unwrap_err();
        assert!(message.contains("invalid duration '3x'"));
        assert!(message.contains("Try values like"));
    }
}
