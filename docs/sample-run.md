# Sample Run

This is the fastest way to understand what `executainer` does.

## Command

```bash
executainer run --lanes 3 --task "Audit this repository for risky coupling."
```

## What Happens

1. `doctor` preflight runs first
2. a detached `tmux` session is created
3. each lane receives a bounded prompt
4. `tmp/executainer/<run-slug>/` is populated with:
   - `inputs/`
   - `prompts/`
   - `captures/`
   - `outputs/`
   - `manifest.json`
5. read-only lanes are parsed through the sentinel contract
6. synthesis completes only if parsing succeeds and proposed files do not collide

## What To Look For

- `manifest.json` is the run contract
- `captures/*.pane.txt` are raw operator evidence
- `outputs/*.last-message.txt` are the parsed worker results
- `outputs/synthesis.md` exists only when the run clears ambiguity checks

## Why This Matters

The product is not "three panes showed up."

The product is that a long-running delegation task leaves behind evidence you can inspect, compare, and trust.
