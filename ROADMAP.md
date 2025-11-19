# `timeout` - Timeout CLI Tool

`timeout` is a simple, cross-platform CLI tool written in Rust that runs any
command with a specified timeout. If the command does not finish before the
timeout expires, `timeout` attempts to terminate it gracefully, then forcefully
kills it if needed.

## Value Proposition

**Why choose `timeout` over existing timeout tools?**

`timeout` offers:

- **True cross-platform support**: Works consistently on Linux, macOS, and
  Windows without requiring platform-specific tweaks.
- **Process group handling**: Ensures not just the main process but all spawned
  child processes are terminated.
- **Configurable behavior**: Control grace period, signals, and exit codes for
  better integration into automation workflows.
- **Live output streaming**: See command output in real-time for better
  debugging and monitoring.
- **Prebuilt binaries**: Easy installation without needing Rust or build steps.
- **Modern Rust reliability**: Safe, efficient, and built for maintainability.

## Customer Value

- **DevOps & CI/CD pipelines**: Prevents hanging jobs and ensures build agents
  stay responsive.
- **Developers**: Run tests or scripts with guaranteed time limits for faster
  feedback loops.
- **Automation engineers**: Integrates with scripts and orchestration tools with
  predictable behavior.
- **Cross-platform teams**: One tool, same usage across all OSes.

---

## Competitive Analysis

## Expanded Competitive Analysis

| Feature / Tool                       | Nuewframe timeout (Rust) | GNU timeout              | BusyBox timeout          | timelimit     | Windows PowerShell scripts | Python subprocess wrappers | timeout-cli (Node.js) | shtimeout (Shell) | go-timeout (Go) | systemd service timeouts |
| ------------------------------------ | ----------- | ------------------------ | ------------------------ | ------------- | -------------------------- | -------------------------- | --------------------- | ----------------- | --------------- | ------------------------ |
| Cross-platform (Linux/macOS/Windows) | ✅          | ❌ Linux/macOS only      | ❌ Linux only            | ❌ Linux only | ⚠️ Windows only            | ✅                         | ✅                    | ✅ (POSIX shells) | ✅              | ❌ Linux only            |
| Process group termination            | ✅          | ⚠️ Limited               | ❌                       | ⚠️ Limited    | ⚠️ Limited                 | ⚠️ Limited                 | ⚠️ Limited            | ❌                | ✅              | ✅                       |
| Configurable grace period            | ✅          | ✅                       | ❌                       | ✅            | ⚠️ Limited                 | ✅                         | ✅                    | ⚠️ Limited        | ✅              | ✅                       |
| Custom signals (Unix)                | ✅          | ✅                       | ❌                       | ✅            | ❌                         | ✅                         | ❌                    | ⚠️ Limited        | ✅              | ✅                       |
| Prebuilt binaries                    | ✅          | ✅ (via package manager) | ✅ (via package manager) | ⚠️ Limited    | ❌                         | ❌ (requires runtime)      | ✅ (npm)              | ⚠️ Limited        | ✅              | ❌                       |
| JSON/structured output               | Planned     | ❌                       | ❌                       | ❌            | ❌                         | ✅                         | ✅                    | ❌                | ✅              | ❌                       |
| Live stdout/stderr streaming         | ✅          | ✅                       | ✅                       | ✅            | ✅                         | ✅                         | ✅                    | ✅                | ✅              | ❌                       |
| Config file for defaults             | Planned     | ❌                       | ❌                       | ❌            | ❌                         | ✅                         | ✅                    | ⚠️ Limited        | ✅              | ✅                       |
| Lightweight, single binary           | ✅          | ✅                       | ✅                       | ✅            | ❌                         | ❌                         | ⚠️ Limited            | ✅                | ✅              | ❌                       |
| Actively maintained                  | Planned     | ✅                       | ✅                       | ❌            | ❌                         | ✅                         | ✅                    | ⚠️ Limited        | ✅              | ✅                       |

✅ = fully supported | ⚠️ = partial/limited support | ❌ = not supported

### Where `timeout` Differentiates Strongly:

1. **Cross-platform reliability** — consistent usage and behavior across Linux,
   macOS, and Windows without runtime dependencies.
2. **Full process group termination** — ensures no orphaned child processes
   remain, unlike many wrappers.
3. **Rust performance and safety** — native speed with minimal memory footprint.
4. **Prebuilt, dependency-free binaries** — works out of the box without
   requiring Python, Node.js, or a specific shell environment.
5. **Future extensibility** — structured output, config defaults, and advanced
   termination strategies planned.

## Roadmap (Thin-Slice Features)

### MVP (Already implemented or planned)

1. **Basic timeout execution** — run a command with a fixed timeout.
2. **Cross-platform process handling** — terminate main process and children.
3. **Grace period support** — configurable wait before force kill.
4. **Custom signal support (Unix)** — allow specifying termination signal.
5. **Live stdout/stderr streaming** — display process output in real-time.

---

### Milestones

#### Thin-Slice 1: JSON/Structured Output

* Output execution metadata (exit code, runtime, termination reason) in JSON.
* Allow optional pretty-printing and redirection to file.

#### Thin-Slice 2: Config File for Defaults

* Support `.toutrc` in home directory for default timeout, grace period, signal, etc.
* CLI flags override config file values.

#### Thin-Slice 3: Cross-Platform CI Coverage

* Set up automated tests across Linux, macOS, and Windows.
* Include process group and signal handling tests.

#### Thin-Slice 4: Extensibility Hooks

* Add pre-termination and post-termination hooks.
* Allow external programs/scripts to be triggered.

---

### Post-Parity Enhancements

#### Thin-Slice 5: Plugin Support

* Define a plugin interface (Rust dynamic linking or config-driven commands) for custom behaviors.

#### Thin-Slice 6: Enhanced Output Modes

* Support multiple output formats (YAML, XML) for broader integration.

#### Thin-Slice 7: Timeout Profiles

* Named profiles in config for different workflows (e.g., `ci`, `debug`).

### **MVP (v0.1)**

1. **Basic Timeout Execution** — Run command with fixed timeout and kill if exceeded.
2. **Live Output Streaming** — Inherit and display stdout/stderr in real-time.
3. **Exit Code Forwarding** — Return child's exit code or `124` on timeout.

### **v0.2**

4. **Graceful Termination on Timeout** — Send SIGTERM (Unix) or kill request (Windows) before force-killing.
5. **Process Group Handling** — Kill all child processes spawned by the main command.
6. **Timeout Parsing Improvements** — Support `500ms`, `1.5m`, etc., using `humantime`.

### **v0.3**

7. **Custom Grace Period** — `--grace <duration>` before sending SIGKILL.
8. **Custom Timeout Exit Code** — `--kill-code <int>` to configure timeout code.
9. **Custom Termination Signal (Unix)** — `--signal <SIG>` to choose termination signal.

### **v0.4**

10. **Config File Support** — Defaults stored in `~/.toutrc` (e.g., default grace, signals).
11. **Dry Run Mode** — Show command and settings without executing.
12. **Verbose Logging** — Detailed execution and termination logs.

### **v0.5**

13. **Windows Ctrl-C Forwarding** — Use ConsoleCtrl APIs to send `CTRL_BREAK_EVENT` to process group.
14. **Shell Mode Flag** — `--shell` to auto-wrap command in system shell.
15. **JSON Output Mode** — Structured output for automation (`--json`).

### **v1.0**

16. **Cross-Platform Integration Tests** — Full coverage for Unix/Windows behaviors.
17. **Prebuilt Binaries** — Release for Linux/macOS/Windows via GitHub Actions.
18. **Man Page & Autocompletion** — `man tout` + shell completion scripts for Bash, Zsh, Fish.

---
