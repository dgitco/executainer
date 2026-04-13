# INSTALL

This is the source of truth for getting `executainer` running.

## Requirements

- Rust toolchain with `cargo`
- `tmux`
- Codex CLI available as `codex`

Check them first:

```bash
rustc --version
cargo --version
tmux -V
codex --help
```

If one of those fails, fix that first. `executainer` is intentionally small, so most install trouble is really environment trouble.

## Install From Release

Release archives are published at:

<https://github.com/dgitco/executainer/releases/latest>

Choose the archive for your platform, unpack it, and put `executainer` on your `PATH`.

Example:

```bash
tar -xzf executainer-<platform>.tar.gz
chmod +x executainer
mv executainer /usr/local/bin/executainer
```

## Install From Source

Install directly from Git:

```bash
cargo install --git https://github.com/dgitco/executainer
```

Or, if you prefer a local checkout:

```bash
cargo install --path .
```

## First-Run Verification

Run:

```bash
executainer doctor
```

Healthy output should report these checks as ready:

- runtime
- `tmux`
- `codex_cli`
- writable temp dir
- parser self-check

If `doctor` fails, do not debug lane behavior yet. Fix the environment first.

## First Useful Command

Once `doctor` is green:

```bash
executainer run --lanes 3 --task "Audit this repository for risky coupling."
```

Then inspect the run:

```bash
executainer inspect <run-slug>
```

## Known v1 Boundaries

- `tmux` is the only operator backend
- release binaries do not remove the need for local `tmux` and Codex
- writable lanes are intentionally human-visible and can block synthesis

That is not accidental. v1 is optimizing for trust and operator control, not magic.
