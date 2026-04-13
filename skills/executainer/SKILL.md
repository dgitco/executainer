# executainer

This wrapper is intentionally thin.

## What it does

- Points the user or agent at `INSTALL.md`
- Runs `executainer doctor`
- Suggests a representative `executainer run` invocation

## What it does not do

- It does not own orchestration logic
- It does not parse lane output
- It does not perform synthesis
- It does not replace the CLI surface

## Suggested flow

```bash
executainer doctor
executainer run --lanes 3 --task "Audit this repository for risky coupling."
executainer inspect <run-slug>
```
