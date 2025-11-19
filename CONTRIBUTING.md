# Contributing to `timeout`

Thanks for helping harden `timeout`! The CLI lives under the **Nuewframe** umbrella at NuewLabs Inc., an open-source initiative for automation-friendly tooling. The practices below apply to every Nuewframe crate and CLI. Start with `README.md` for the product overview, then dive into this guide for contributor expectations.

## Development Workflow
1. **Fork + branch** from `main` (semantic prefixes like `feat/<topic>`, `fix/<topic>`, `docs/<topic>` keep history aligned with Conventional Commits).
2. **Install toolchain** – we target the latest stable Rust. Run `rustup update` prior to opening a PR.
3. **Run the basics** before every push:
   ```bash
   cargo fmt --all
   cargo clippy --all-targets -- -D warnings
   cargo test --workspace --bins
   cargo test --test e2e
   cargo audit
   ```
    > `cargo audit` is provided by [cargo-audit](https://github.com/RustSec/rustsec/tree/main/cargo-audit). Install it once via `cargo install cargo-audit --locked` before running the command above.

4. **Open a PR** with a concise description, testing evidence, and any screenshots/logs that help reviewers.

## Local Development

To build and run the project locally:

```bash
# Build release binary
cargo build --release

# Run with arguments
cargo run -- 5s -- echo "Hello world"
```

## Coding Guidelines
- Prefer small, well-tested changes. If you must touch unrelated files, explain why.
- Use async-aware patterns (Tokio) and avoid blocking calls inside async contexts.
- Keep logging user-friendly: default to quiet operation and gate verbose traces behind flags.
- Document any behavior that intentionally diverges from GNU `timeout`.

## Commit Hygiene
- Follow Conventional Commits when possible (`feat:`, `fix:`, `docs:`, etc.).
- Squash fixups before merging; we squash PRs by default.

## Tests
- Unit tests live alongside the code (`src/main.rs`).
- End-to-end coverage is under `tests/e2e.rs` and uses `assert_cmd` to build fresh binaries.
- Add regression tests whenever you fix a bug or add a feature.

## Releases
- Tag `vX.Y.Z` to trigger the release workflow. CI will build macOS, Linux, and Windows artifacts, run the full suite, and publish to GitHub Releases.

## Questions?
Open a discussion or reach out via `nuewframe@nuewlabs.com`.
