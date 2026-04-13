# INSTALL

This is the bootstrap source of truth for `executainer`.

## Requirements

- Rust toolchain with `cargo`
- `tmux`
- Codex CLI available as `codex`

## Preflight

```bash
rustc --version
cargo --version
tmux -V
codex --help
```

## Install

Use one path only:

```bash
cargo install --path .
```

## First Run

```bash
executainer doctor
```

If `doctor` reports any failing checks, fix those first. Do not try to run lanes before preflight is green.
