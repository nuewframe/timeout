use assert_cmd::cargo::cargo_bin_cmd;
use predicates::prelude::*;

fn timeout_cmd() -> assert_cmd::Command {
    cargo_bin_cmd!("timeout")
}

// Cross-platform test helpers that return strings for dynamic construction
#[cfg(unix)]
fn sleep_args(seconds: &str) -> Vec<String> {
    vec!["sleep".to_string(), seconds.to_string()]
}

#[cfg(windows)]
fn sleep_args(seconds: &str) -> Vec<String> {
    vec![
        "powershell".to_string(),
        "-Command".to_string(),
        format!("Start-Sleep {}", seconds),
    ]
}

#[cfg(unix)]
fn echo_args(text: &str) -> Vec<String> {
    vec!["echo".to_string(), text.to_string()]
}

#[cfg(windows)]
fn echo_args(text: &str) -> Vec<String> {
    vec![
        "cmd".to_string(),
        "/c".to_string(),
        "echo".to_string(),
        text.to_string(),
    ]
}

#[test]
fn test_version() {
    timeout_cmd()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("timeout v"))
        .stdout(predicate::str::contains("Copyright (c) 2025 NuewLabs Inc"));
}

#[test]
fn test_invalid_duration() {
    let mut cmd = timeout_cmd();
    cmd.args(&["invalid", "--"]);
    cmd.args(echo_args("test"));
    cmd.assert()
        .failure()
        .stderr(predicate::str::is_match(r"invalid duration '[^']+'").unwrap());
}

#[test]
fn test_arithmetic_duration_invalid() {
    let mut cmd = timeout_cmd();
    cmd.args(&["3x", "--"]);
    cmd.args(echo_args("done"));
    cmd.assert()
        .failure()
        .stderr(predicate::str::is_match(r"invalid duration '[^']+'").unwrap());
}

#[test]
fn test_echo_success() {
    let mut cmd = timeout_cmd();
    cmd.args(&["1s", "--"]);
    cmd.args(echo_args("hello"));
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("hello"));
}

#[test]
fn test_timeout_expires() {
    let mut cmd = timeout_cmd();
    cmd.args(&["0.1s", "--"]);
    cmd.args(sleep_args("1"));
    cmd.assert()
        .code(124)
        .stderr(predicate::str::contains("Command timed out"));
}

#[test]
fn test_arithmetic_command() {
    let mut cmd = timeout_cmd();
    cmd.args(&["1s", "--"]);
    cmd.args(echo_args("$((2*2))"));
    cmd.assert()
        .success()
        .stdout(predicate::str::is_match(r"\$\(\(2\*2\)\)").unwrap());
}

#[cfg(unix)]
#[test]
fn test_verbose_prints_diagnostics() {
    timeout_cmd()
        .args(&["--verbose", "0.5s", "--", "true"])
        .assert()
        .success()
        .stderr(predicate::str::contains("Starting command:"))
        .stderr(predicate::str::contains("Timeout:"));
}

#[cfg(windows)]
#[test]
fn test_verbose_prints_diagnostics() {
    timeout_cmd()
        .args(&["--verbose", "0.5s", "--", "cmd", "/c", "exit", "0"])
        .assert()
        .success()
        .stderr(predicate::str::contains("Starting command:"))
        .stderr(predicate::str::contains("Timeout:"));
}

#[test]
fn test_quiet_suppresses_timeout() {
    let mut cmd = timeout_cmd();
    cmd.args(&["--quiet", "0.1s", "--"]);
    cmd.args(sleep_args("1"));
    cmd.assert()
        .code(124)
        .stderr(predicate::str::is_match(r"(?s)^\s*$").unwrap());
}

#[test]
fn test_kill_after_zero_logs() {
    let mut cmd = timeout_cmd();
    cmd.args(&["-k", "0s", "0.1s", "--"]);
    cmd.args(sleep_args("1"));
    cmd.assert()
        .code(124)
        .stderr(predicate::str::contains("kill-after duration is 0"));
}

#[cfg(unix)]
#[test]
fn test_signal_flag_int() {
    timeout_cmd()
        .args(&[
            "-s",
            "INT",
            "0.1s",
            "--",
            "sh",
            "-c",
            "trap 'echo got-int; exit 0' INT; trap 'echo got-term; exit 0' TERM; sleep 5",
        ])
        .assert()
        .code(124)
        .stdout(predicate::str::contains("got-int"))
        .stderr(predicate::str::contains("Command timed out"));
}
