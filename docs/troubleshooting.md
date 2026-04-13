# Troubleshooting

## `doctor` says `tmux` failed

Install `tmux` and re-run:

```bash
tmux -V
executainer doctor
```

## `doctor` says `codex_cli` failed

Make sure the Codex CLI is installed and available as `codex`:

```bash
codex --help
```

## `run` exits with `doctor preflight failed`

That means the machine is not ready yet. Fix the failed `doctor` checks first.

## `run` exits with `requires review`

That is expected when:

- lane parsing failed
- multiple lanes proposed the same file
- a writable lane is still waiting on a human

Inspect the run:

```bash
executainer inspect <run-slug> --json
```

Then open the run directory under `tmp/executainer/<run-slug>/`.

## I do not see `synthesis.md`

That means synthesis was intentionally blocked. This is a safety feature, not a bug.

## I cleaned a run but the tmux session is still around

Run:

```bash
tmux ls
```

If the session still exists, kill it manually:

```bash
tmux kill-session -t executainer-<run-slug>
```
