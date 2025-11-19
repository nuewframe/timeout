use assert_cmd::cargo::cargo_bin_cmd;
use regex::Regex;
use std::process::Output;

fn run_timeout(args: &[&str]) -> Output {
    let mut cmd = cargo_bin_cmd!("timeout");
    cmd.args(args)
        .output()
        .expect("Failed to run timeout command")
}

#[derive(Debug, Clone)]
struct TestCase {
    args: Vec<&'static str>,
    expected_success: bool,
    expected_code: Option<i32>,
    expected_stdout_patterns: Vec<&'static str>,
    expected_stderr_patterns: Vec<&'static str>,
}

macro_rules! generate_tests {
    ($($name:ident: $case:expr),* $(,)?) => {
        $(
            #[test]
            fn $name() {
                let case = $case;
                let output = run_timeout(&case.args);
                if let Some(code) = case.expected_code {
                    assert_eq!(output.status.code(), Some(code), "for args {:?}", case.args);
                } else if case.expected_success {
                    assert!(output.status.success(), "for args {:?}", case.args);
                } else {
                    assert!(!output.status.success(), "for args {:?}", case.args);
                }
                let stdout = String::from_utf8_lossy(&output.stdout);
                let args = case.args;
                for pattern in &case.expected_stdout_patterns {
                    assert!(Regex::new(pattern).unwrap().is_match(&stdout),
            "stdout missing pattern /{pattern}/; stdout: '{stdout}', args: {args:?}");
                }
                let stderr = String::from_utf8_lossy(&output.stderr);
                for pattern in &case.expected_stderr_patterns {
                    assert!(Regex::new(pattern).unwrap().is_match(&stderr),
            "stderr missing pattern /{pattern}/; stderr: '{stderr}', args: {args:?}");
                }
            }
        )*
    };
}

generate_tests! {
    test_version: TestCase {
        args: vec!["--version"],
        expected_success: true,
        expected_code: None,
        expected_stdout_patterns: vec!["timeout v", r"Copyright \(c\) 2025 NuewLabs Inc"],
        expected_stderr_patterns: vec![],
    },
    test_echo_success: TestCase {
        args: vec!["1s", "--", "echo", "hello"],
        expected_success: true,
        expected_code: None,
        expected_stdout_patterns: vec!["hello"],
        expected_stderr_patterns: vec![],
    },
    test_timeout_expires: TestCase {
        args: vec!["0.1s", "--", "sleep", "1"],
        expected_success: false,
        expected_code: Some(124),
        expected_stdout_patterns: vec![],
        expected_stderr_patterns: vec!["Command timed out"],
    },
    test_arithmetic_duration_invalid: TestCase {
        args: vec!["3x", "--", "echo", "done"],
        expected_success: false,
        expected_code: None,
        expected_stdout_patterns: vec![],
        expected_stderr_patterns: vec![r"invalid duration '[^']+'"],
    },
    test_arithmetic_command: TestCase {
        args: vec!["1s", "--", "echo", "$((2*2))"],
        expected_success: true,
        expected_code: None,
        expected_stdout_patterns: vec![r"\$\(\(2\*2\)\)"],
        expected_stderr_patterns: vec![],
    },
    test_invalid_duration: TestCase {
        args: vec!["invalid", "--", "echo", "test"],
        expected_success: false,
        expected_code: None,
        expected_stdout_patterns: vec![],
        expected_stderr_patterns: vec![r"invalid duration '[^']+'"],
    },
    test_verbose_prints_diagnostics: TestCase {
        args: vec!["--verbose", "0.5s", "--", "true"],
        expected_success: true,
        expected_code: None,
        expected_stdout_patterns: vec![],
        expected_stderr_patterns: vec![r"Starting command:", r"Timeout:"],
    },
    test_quiet_suppresses_timeout: TestCase {
        args: vec!["--quiet", "0.1s", "--", "sleep", "1"],
        expected_success: false,
        expected_code: Some(124),
        expected_stdout_patterns: vec![],
        expected_stderr_patterns: vec![r"(?s)^\s*$"],
    },
    test_kill_after_zero_logs: TestCase {
        args: vec!["-k", "0s", "0.1s", "--", "sleep", "1"],
        expected_success: false,
        expected_code: Some(124),
        expected_stdout_patterns: vec![],
        expected_stderr_patterns: vec!["kill-after duration is 0"],
    },
    test_signal_flag_int: TestCase {
        args: vec!["-s","INT","0.1s","--","sh","-c","trap 'echo got-int; exit 0' INT; trap 'echo got-term; exit 0' TERM; sleep 5"],
        expected_success: false,
        expected_code: Some(124),
        expected_stdout_patterns: vec!["got-int"],
        expected_stderr_patterns: vec!["Command timed out"],
    },
}
