# Executainer v1 Architecture

## Core Model

`executainer` is a run-oriented CLI. Each run produces a deterministic archive rooted at `tmp/executainer/<slug>/`.

The architecture is built around one idea: keep execution lanes clean, keep the orchestrator thin, and keep evidence durable.

The run archive stores:

- `inputs/` for generated lane scripts and task inputs
- `prompts/` for rendered lane prompts and synthesis prompts
- `captures/` for raw `tmux capture-pane` evidence
- `outputs/` for last-message files, raw stdout, done markers, and synthesis output
- `manifest.json` for the run contract

## Execution Layers

### Read-only lanes

Read-only lanes use `codex exec`.

- non-interactive
- good for deterministic prompt injection
- easy to archive and parse
- used for audit, planning, and analysis
- especially useful when you want multiple viewpoints without cross-contaminating reasoning

### Writable lanes

Writable lanes are explicit opt-in and run as interactive Codex sessions inside `tmux`.

- approval-heavy work stays visible to the operator
- the user can attach to the pane and intervene
- writable scope is recorded in `manifest.json`
- implementation work stays chunked into bounded units instead of becoming one giant session with a giant memory burden

### tmux backend

`tmux` is the operator substrate.

- pane visibility
- attach and inspect
- raw pane capture
- manual intervention for writable work

### Thin orchestrator

The orchestrator is intentionally small.

Its job is to:

- assign work
- inspect outputs
- compare evidence
- detect collisions
- decide what happens next

Its job is not to become a second implementation lane or to carry the full evolving context of every worker. That is how orchestration turns into context soup.

## Parser And Synthesis

The parser trusts only the sentinel block. Raw pane capture is evidence, not truth.

Synthesis consumes:

- parsed lane results
- raw capture metadata
- run manifest

Synthesis is blocked when parsing fails or when multiple lanes propose the same file.

That is a product decision as much as an implementation detail. The system should stop on ambiguity instead of smoothing over it with a fake feeling of progress.

## Future Track

Approval evaluation is a future track, not a v1 feature.

The intended direction is an independent evaluator that reviews approval requests, classifies risk, and recommends whether a request should stay human-gated, be allowed once, or become a persistent allow candidate.

Another future track is compact-safe control state:

- markdown-first knowledge sync
- resumable orchestration after compaction
- lightweight operator interruption flows

The target outcome is long-running execution that stays navigable, steerable, and objective even when the work is large enough to break ad hoc agent workflows.
