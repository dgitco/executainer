# executainer

`executainer` is a CLI-first trusted parallel delegation tool for terminal power users. It is not a multi-pane toy. The point is faster, safer, more reproducible delegation.

## Agent-First Install

Fetch and follow instructions from `<raw INSTALL.md URL>`.

That is the default onboarding path. `INSTALL.md` is the source of truth for bootstrap and first-run verification.

## What v1 Does

- Runs read-only delegation lanes with `codex exec`
- Uses `tmux` as the visibility and operator-control substrate
- Separates writable work into explicit opt-in interactive lanes
- Archives prompts, captures, outputs, and a deterministic `manifest.json`
- Stops before synthesis when parse failures or conflicting file proposals show up

## Public CLI

```bash
executainer doctor
executainer run --lanes 3 --task "Audit this repository for risky coupling."
executainer inspect <run-slug>
executainer clean <run-slug> --yes
```

## Manual Install Fallback

If you are not using the agent-first path, install Rust, `tmux`, and the Codex CLI, then:

```bash
cargo install --path .
executainer doctor
```
