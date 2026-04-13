# Executainer v1 PRD

## Product Definition

`executainer` v1 is a trusted parallel delegation CLI for terminal power users.

The product promise is not "run a bunch of panes." The promise is that delegation becomes easier to trust because it is approval-friendly, reproducible, inspectable, and conflict-aware.

## v1 Wedge

v1 is `tmux-only`, but `tmux` is the operating layer, not the product identity.

- Read-only analysis, audit, planning, and adversarial review lanes run with `codex exec`
- Writable work is explicit opt-in and runs in interactive `tmux` lanes
- A deterministic run archive is always written under `tmp/executainer/<slug>/`

## Success Criteria

Success is defined by user outcomes, not by the mere fact that panes open.

- Better wall-clock time than a single-agent baseline
- Less operator intervention across repeated workflows
- A high synthesis usable rate
- Reliable conflicting output detection
- High first-run success rate for setup and execution

## Non-Goals

- Full sandbox isolation in v1
- Non-`tmux` backends in v1
- Wrapper-owned orchestration logic
- Automatic approval evaluation in v1

## Future Direction

The long-term direction is a trust layer for delegation.

That includes a future independent approval evaluator that classifies approval requests, recommends allow-once or allow-persist paths, and learns the difference between stable requests and risky ones without collapsing the human-in-the-loop boundary.
