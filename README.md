# executainer

`executainer` is a Rust CLI for running agent work in isolated lanes instead of one giant shared context.

It is for the person who already knows the painful manual version:

- open another session
- restate the task
- copy just enough context over
- keep lane notes somewhere else
- hope the workers do not step on each other

`executainer` turns that into a repeatable workflow with visible panes, deterministic run artifacts, and a hard stop when lane output is ambiguous.

## Who This Is For

The sharpest v1 user is an engineer running long or high-risk agent work where shared context makes results worse.

Good first workflows:

- parallel repository audits
- adversarial reviews from multiple viewpoints
- migration planning with isolated read-only lanes
- implementation handoffs where writable work needs human visibility

If you want a giant all-knowing control plane, this is not that.

## Why It Exists

Most agent tools still assume the answer is more shared context.

That works until every worker sees too much, overreads, drifts, repeats work, and starts sounding confident about the wrong thing.

`executainer` takes the opposite bet:

- keep each lane bounded
- keep the orchestrator thin
- trust recorded evidence
- block on ambiguity instead of smoothing it over

## What v1 Does Today

- runs read-only delegation lanes with `codex exec`
- uses `tmux` as the operator visibility layer
- supports explicit writable lanes for human-supervised work
- archives prompts, pane captures, outputs, and a deterministic `manifest.json`
- blocks synthesis when parsing fails or lanes collide on proposed files

## What It Does Not Do

Not in v1:

- full sandbox isolation
- automatic approval classification
- non-`tmux` backends
- hosted orchestration

Those are future directions, not hidden promises.

## Install

The full install guide lives in [INSTALL.md](https://raw.githubusercontent.com/dgitco/executainer/main/INSTALL.md).

Fast paths:

1. Download a release binary from [GitHub Releases](https://github.com/dgitco/executainer/releases/latest)
2. Install directly from Git:

```bash
cargo install --git https://github.com/dgitco/executainer
```

3. Or install from local source:

```bash
cargo install --path .
```

## First Run

Verify the local environment:

```bash
executainer doctor
```

Expected healthy checks:

- runtime
- `tmux`
- `codex_cli`
- writable temp dir
- parser self-check

Then run a real task:

```bash
executainer run --lanes 3 --task "Audit this repository for risky coupling."
```

Inspect the result:

```bash
executainer inspect <run-slug>
```

Clean it up:

```bash
executainer clean <run-slug> --yes
```

## Example Run

See [docs/sample-run.md](docs/sample-run.md) for a concrete walkthrough and what the output artifacts look like.

## Release Trust

This repo is meant to be boring in the right places.

- CI runs formatting, linting, tests, and build checks
- tagged releases publish release archives and checksums
- docs call out the real support boundary instead of pretending v1 is more complete than it is

## Support Matrix

Current release target:

- macOS via release binary or source build
- Linux via release binary or source build
- local `tmux`
- local Codex CLI available as `codex`

If your machine already has Rust, `tmux`, and Codex, time to first useful run should be under 5 minutes.

## Troubleshooting

Common issues and operator mistakes live in [docs/troubleshooting.md](docs/troubleshooting.md).

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

## Security

See [SECURITY.md](SECURITY.md).
