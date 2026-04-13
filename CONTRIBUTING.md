# Contributing

Thanks for helping make `executainer` better.

## Before You Open A PR

1. Read the README and INSTALL flow first.
2. Keep changes narrow. This repo is intentionally small.
3. Prefer explicit behavior over clever abstractions.
4. If the change affects operator trust, docs are part of the change.

## Local Development

Prerequisites:

- Rust toolchain
- `tmux`
- Codex CLI available as `codex`

Run the core checks before you push:

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

If you are changing onboarding or release docs, also walk through:

```bash
cargo run -- --help
cargo run -- doctor --json
```

`doctor` validates your local machine, so it is expected to fail if `tmux` or `codex` is missing.

## Pull Requests

- Explain the user-facing problem first.
- Include verification commands and results.
- Call out docs changes when behavior changes.
- Keep follow-up work out of scope unless it is in the direct blast radius.

## Scope Discipline

This project wins by staying legible.

- Small CLI surface
- Deterministic artifacts
- Thin orchestrator
- Clear failure states

If a change makes the tool feel smarter but less inspectable, that is usually a bad trade.
