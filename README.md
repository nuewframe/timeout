# `timeout` - A Timeout Command-Line (CLI) Tool

`timeout` is a simple, cross-platform CLI tool written in Rust that runs any command with a specified timeout. If the
command does not finish before the timeout expires, `timeout` attempts to terminate it gracefully, then forcefully kills it
if needed.

## Value Proposition

**Why choose `timeout` over existing timeout tools?**

`timeout` offers:

* **True cross-platform support**: Works consistently on Linux, macOS, and Windows without requiring platform-specific
  tweaks.
* **Process group handling**: Ensures not just the main process but all spawned child processes are terminated.
* **Configurable behavior**: Control grace period, signals, verbosity, and exit codes for better integration into
  automation workflows.
* **Live output streaming**: See command output in real-time for better debugging and monitoring.
* **Prebuilt binaries**: Easy installation without needing Rust or build steps.
* **Modern Rust reliability**: Safe, efficient, and built for maintainability.

## Customer Value

* **DevOps & CI/CD pipelines**: Prevents hanging jobs and ensures build agents stay responsive.
* **Developers**: Run tests or scripts with guaranteed time limits for faster feedback loops.
* **Automation engineers**: Integrates with scripts and orchestration tools with predictable behavior.
* **Cross-platform teams**: One tool, same usage across all OSes.

---

## Installation

### Quick Install (Linux/macOS)

```bash
curl -fsSL https://raw.githubusercontent.com/nuewframe/timeout/main/install.sh | bash
```

This will download the latest release binary and install it to `~/.local/bin`. Make sure this directory is in your `PATH`.

### Quick Install (Windows)

```powershell
irm https://raw.githubusercontent.com/nuewframe/timeout/main/install.ps1 | iex
```

This will download the latest release binary and install it to `%LOCALAPPDATA%\Programs\timeout`, automatically adding it to your PATH.

### From Prebuilt Binaries

Download the latest release for your platform from the [Releases](https://github.com/nuewframe/timeout/releases) page, then
place the binary in your system PATH.

### Using Cargo

```bash
cargo install nuewframe-timeout
```

The binary will be installed as `timeout` in your Cargo bin directory.

### From Source

```bash
git clone https://github.com/nuewframe/timeout.git
cd timeout
cargo build --release
# Binary will be in target/release/timeout
```

---

## Usage

Basic usage:

```bash
timeout 30s -- cargo test --package test_package --test test_case
```

**Examples:**

* Run a command with a 10-second timeout:

  ```bash
  timeout 10s -- ./long_running_script.sh
  ```
* Evaluate arithmetic or shell expansions (GNU-compatible):

  ```bash
  timeout 1s -- sh -c 'echo $((2 + 2))'
  ```
  > `timeout` intentionally mirrors GNU timeout: expressions are evaluated by your shell, not by the CLI itself. Wrap the payload in `sh -c` (or `cmd /C` on Windows) when you need arithmetic or globbing support.
* Customize the graceful signal and kill-after window:

  ```bash
  timeout -s INT -k 250ms 1s -- sh -c 'trap "echo got-int" INT; sleep 5'
  ```
  > `-s` accepts Unix signal names or numbers, while `-k` controls how long `timeout` waits before escalating to `SIGKILL`. Use `-k 0` to skip the grace period.
* Adjust verbosity:

  ```bash
  timeout --quiet 5s -- sleep 10     # fully silent diagnostics
  timeout --verbose 5s -- cargo test # emit start/timeout details
  ```
* Show version:

  ```bash
  timeout --version
  ```

**Flags:**

```
-V, --version         Show version
-h, --help            Print help
-v, --verbose         Print detailed diagnostics (overrides --quiet)
-q, --quiet           Suppress timeout diagnostics
-s, --signal <SIG>    Unix: send SIG (name or number) before escalating to SIGKILL
-k, --kill-after <D>  Grace period before SIGKILL (default 5s, use 0 for immediate)
```

---

## Unix Signals Reference

Common signals used with `--signal` / `-s` on Unix-like systems:

| Signal | Number | Description |
|--------|--------|-------------|
| `HUP`  | 1      | Hangup (reload configuration) |
| `INT`  | 2      | Interrupt (Ctrl+C) |
| `QUIT` | 3      | Quit (Ctrl+\) |
| `KILL` | 9      | Force Kill (cannot be caught) |
| `TERM` | 15     | Termination (default) |
| `USR1` | 10/30* | User-defined signal 1 |
| `USR2` | 12/31* | User-defined signal 2 |

> \* Numbers for `USR1`/`USR2` vary by architecture/OS.

For a complete list, see [man7.org: signal(7)](https://man7.org/linux/man-pages/man7/signal.7.html).

---

## Exit Codes

* **0**: Command finished successfully
* **Non-zero**: Command failed, returns same exit code as the child process
* **124**: Timeout reached, process terminated

---

## Windows Support

While `timeout` works natively on Windows, the operating system does not support POSIX-style signals (like `SIGTERM` or `SIGINT`).

* **Signals**: The `--signal` / `-s` flag is ignored on Windows. Windows uses Console Control Events (`CTRL_C_EVENT`, `CTRL_BREAK_EVENT`) which behave differently.
* **Termination**: If a command times out, `timeout` attempts a graceful termination where possible, but often defaults to a forced kill (similar to `TerminateProcess`) after the grace period.
* **Process Groups**: `timeout` attempts to kill the entire process tree using Job Objects where applicable.

For more details on Windows process control:
* [Console Control Handlers](https://learn.microsoft.com/windows/console/console-control-handlers)
* [TerminateProcess function](https://learn.microsoft.com/windows/win32/api/processthreadsapi/nf-processthreadsapi-terminateprocess)

---

## Deployment

`timeout` can be integrated into:

* **CI/CD pipelines**: Add as a step to prevent hanging builds.
* **Automation scripts**: Wrap long-running commands to ensure predictable completion.
* **Local development**: Quickly terminate runaway processes during testing.

For Docker environments:

```dockerfile
FROM rust:latest
RUN cargo install nuewframe-timeout
ENTRYPOINT ["timeout"]
```

For CI usage (GitHub Actions example):

```yaml
- name: Install timeout
  run: cargo install nuewframe-timeout

- name: Run tests with timeout
  run: timeout 5m -- cargo test
```


## Roadmap

See [ROADMAP.md](ROADMAP.md) for detailed development roadmap.

---

## License

MIT License. See [LICENSE](LICENSE) for details.

---