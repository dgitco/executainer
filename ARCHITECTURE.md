# Executainer v1 Architecture

## Core Model

`executainer` is a run-oriented CLI. Each run produces a deterministic archive rooted at `tmp/executainer/<slug>/`.

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

### Writable lanes

Writable lanes are explicit opt-in and run as interactive Codex sessions inside `tmux`.

- approval-heavy work stays visible to the operator
- the user can attach to the pane and intervene
- writable scope is recorded in `manifest.json`

### tmux backend

`tmux` is the operator substrate.

- pane visibility
- attach and inspect
- raw pane capture
- manual intervention for writable work

## Parser And Synthesis

The parser trusts only the sentinel block. Raw pane capture is evidence, not truth.

Synthesis consumes:

- parsed lane results
- raw capture metadata
- run manifest

Synthesis is blocked when parsing fails or when multiple lanes propose the same file.

## Future Track

Approval evaluation is a future track, not a v1 feature.

The intended direction is an independent evaluator that reviews approval requests, classifies risk, and recommends whether a request should stay human-gated, be allowed once, or become a persistent allow candidate.
