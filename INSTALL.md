# INSTALL

This is the bootstrap source of truth for `executainer`.

`executainer` is the execution layer for agents that need to stay on track. The install path is intentionally boring. That is a feature.

## Requirements

- Rust toolchain with `cargo`
- `tmux`
- Codex CLI available as `codex`

## Preflight

Verify the operator environment before you install anything:

```bash
rustc --version
cargo --version
tmux -V
codex --help
```

If one of these fails, stop there and fix it first. Do not debug agent lanes on top of a broken base environment.

## Install

Use one path only:

```bash
cargo install --path .
```

## First Run

Run the environment check before starting any delegation work:

```bash
executainer doctor
```

`doctor` is the first-run guardrail. If it reports failing checks, fix those first.

## Then Run Something Real

Once `doctor` is green:

```bash
executainer run --lanes 3 --task "Audit this repository for risky coupling."
```

That gives you the real workflow immediately:

- isolated lanes
- visible execution through `tmux`
- deterministic run artifacts
- a thin operator loop instead of one giant polluted session
