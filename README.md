# executainer

For agents that get lost, overread, and burn tokens like it's free.

Executainer is the execution layer for agents that need to stay on track.

Give each agent a clean execution environment, keep context short, and make long refactors navigable.

## What It Is

`executainer` is a CLI-first execution layer for running agent work in isolated lanes instead of dumping one giant codebase, one giant context window, and one giant problem onto a single session.

It is for people who already know the painful manual version of this workflow. Open another session. Split the task again. Copy context over. Keep notes somewhere else. Try not to lose the thread. Do that 80 times in a week and software starts to feel absurd.

Executainer turns that into a repeatable system.

## Why It Exists

Most agent tooling still assumes the answer is a bigger harness, a fancier planner, or more shared context.

That works right up until it doesn't.

When every agent sees too much, they overread, drift, repeat work, import irrelevant context, and burn tokens on the wrong problem. Planning gets biased by whichever context blob was loaded last. Audits get contaminated by implementation details they should not have seen. Large refactors stop being engineering and start becoming session babysitting.

Executainer takes the opposite approach:

- isolate the work
- keep the goal sharp
- keep the context short
- record the evidence
- let the orchestrator stay thin

That is the whole game.

## Why This Architecture Wins

### More objective planning and audits

Planning and review work gets stronger when multiple lanes can evaluate the same system from different directions without sharing contaminated context. That lowers bias, produces cleaner disagreement, and makes it easier to compare conclusions instead of averaging mush.

### Better implementation accuracy

Implementation quality goes up when work is broken into smaller, explicit pieces with clearer boundaries. Agents do better when they can hold the real task in context instead of swimming through unrelated files and previous reasoning.

### Lower token burn

The orchestrator does not need to carry the full implementation state of every lane. It only needs enough context to assign work, inspect evidence, detect collisions, and decide what happens next. That keeps the central session cheap, stable, and easier to steer.

### Better long-running navigation

Large migrations and refactors fail when the system loses its place. Executainer is built to keep the work navigable over time, not just impressive in a single demo run.

## Where It Shines

Executainer is useful on small tasks too. You can save time, save tokens, and keep the goal cleaner even on modest audits or short implementation bursts.

But its real edge shows up when the work gets long, large, or weird:

- multi-step database refactors
- subsystem migrations
- adversarial audits
- planning from multiple viewpoints
- repository-wide cleanup work
- high-risk changes where context drift quietly destroys correctness

This execution style has already been used to drive large refactors across massive codebases, including long-running financial-platform work, physical computing coding platforms, and database redesign efforts that would otherwise require hundreds of manually managed sessions.

## What v1 Does

- Runs read-only delegation lanes with `codex exec`
- Uses `tmux` as the visibility and operator-control substrate
- Separates writable work into explicit opt-in interactive lanes
- Archives prompts, captures, outputs, and a deterministic `manifest.json`
- Stops before synthesis when parse failures or conflicting file proposals show up

## How It Works

### 1. Keep lanes isolated

Each lane gets a bounded job. Not a vibe. Not a giant shared brainstorm. A bounded job.

### 2. Keep the orchestrator thin

The main session should not become a giant memory dump. It should coordinate, inspect, compare, and decide. That separation is what keeps large runs steerable.

### 3. Trust evidence, not theater

Executainer records prompts, pane captures, lane outputs, and the run contract in a deterministic archive. Raw output is evidence. Parsed sentinels are the trusted interface.

### 4. Block on ambiguity

If parsing fails or multiple lanes propose the same file, synthesis stops. Not glamorous. Very important.

## Why Not The Usual Agent Stack

Executainer is not trying to be a full-stack harness for everything.

It is not trying to become the center of your entire development environment.

It is not trying to win by owning more orchestration logic than necessary.

The point is simpler:

Give agents the minimum execution structure they need to stay accurate at scale.

That means fewer magical abstractions, fewer giant shared contexts, less orchestration bloat, and a much clearer line between coordination and execution.

## Public CLI

```bash
executainer doctor
executainer run --lanes 3 --task "Audit this repository for risky coupling."
executainer inspect <run-slug>
executainer clean <run-slug> --yes
```

## Agent-First Install

Fetch and follow instructions from `<raw INSTALL.md URL>`.

That is the default onboarding path. `INSTALL.md` is the source of truth for bootstrap and first-run verification.

## Manual Install Fallback

If you are not using the agent-first path, install Rust, `tmux`, and the Codex CLI, then:

```bash
cargo install --path .
executainer doctor
```

## Future Direction

The long-term direction is not "more magic." It is better execution infrastructure.

That includes:

- stronger isolation than v1
- more deliberate approval handling
- live markdown knowledge sync for compact-safe orchestration
- operator interrupts and mid-run correction flows
- lightweight ways to ask, redirect, or clarify without polluting the main control context

In other words, agents that can stay precise for a very long time without turning the whole run into context soup.
