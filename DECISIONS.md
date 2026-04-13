# Decisions

## Why `codex exec`

`codex exec` is the right v1 engine for read-only lanes because it is non-interactive and scriptable. That gives us reproducible prompts, stable cwd control, and machine-collectable results.

## Why `tmux`

`tmux` is the right v1 operator layer because trust is not only about correctness. It is also about visibility. The operator needs panes, attachability, capture, and an intervention point when writable work or approvals show up.

## Why Rust

Rust matches the product position better than a quick script. This tool wants to feel deterministic, typed, and shippable as a single binary.

## Why not `codex review`

`codex review` is great for diff review, but too narrow for general delegation orchestration. `executainer` is broader than review.

## Why not `codex fork` or `codex resume`

Those are interesting future primitives, especially for branchable lane context. They are not the right v1 wedge because session lifecycle and reproducibility get more complex immediately.

## Why not direct API orchestration

Direct API orchestration may become attractive later. In v1 the Codex CLI already gives us cwd, sandbox, approval, and session semantics. Rebuilding those early is the wrong place to spend product energy.
