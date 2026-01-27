# Ralph

A CLI tool that runs AI coding tools iteratively on tasks with PRD/PROGRESS files.

## Build & Install

```bash
# Build
cargo build --release

# Install to /Users/filipe/Scripts
make install
```

## Usage

```bash
ralph --iterations <N> [--dir <DIR> | --task <TASK>] --tool <TOOL> [--model <MODEL>] [--variant <VARIANT>]
```

### Arguments

| Flag | Short | Description |
|------|-------|-------------|
| `--iterations` | `-i` | Number of iterations to run |
| `--dir` | `-d` | Directory path for task |
| `--task` | `-t` | PRD file path or task name (resolves to `.ai/tasks/{name}/PRD.md`) |
| `--tool` | `-T` | Tool to use: `opencode` (oc), `claude` (cc), `codex` |
| `--model` | `-m` | Model name or alias (optional) |
| `--variant` | `-v` | OpenCode variant (only valid with `--tool opencode`) |

> **Note:** Either `--dir` or `--task` must be provided. If both are used, `--task` takes precedence and a warning is shown.

### Examples

```bash
# Run using task name (resolves to .ai/tasks/my-task/PRD.md)
ralph -i 5 -t my-task -T oc

# Run using directory path
ralph -i 5 -d ./path/to/task -T cc

# Use short aliases and model
ralph -i 10 -t my-task -T oc -m opus

# Run opencode with variant
ralph -i 5 -t my-task -T oc -v fast
```

### Model Aliases

**OpenCode:**
- `opus` -> `anthropic/claude-opus-4-5`
- `sonnet` -> `anthropic/claude-sonnet-4-5`
- `haiku` -> `anthropic/claude-haiku-4-5`
- `codex` -> `openai/gpt-5.2-codex`

**Codex:**
- `codex` -> `gpt-5.2-codex`
- `codex-mini` -> `gpt-5.1-codex-mini`
- `codex-max` -> `gpt-5.1-codex-max`

## How It Works

1. Discovers PRD and PROGRESS files in the task directory
2. Confirms files with user
3. Runs the selected AI tool with a structured prompt
4. Repeats for N iterations or until `<promise>COMPLETE</promise>` is detected
