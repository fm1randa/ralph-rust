# PRD: OpenCode Variant Flag

## Goal
Add a `--variant` flag (short `-v`) to the `ralph` CLI that applies only to the `opencode` tool. The flag should pass through to the `opencode` command and be rejected for other tools.

## Background
OpenCode supports model variants that can be listed via `opencode models --verbose` (filterable with `jq`). The `ralph` CLI should expose a variant option specifically for `opencode` runs without affecting other tools.

## Requirements
- Add a new CLI flag `--variant` with shorthand `-v`.
- The flag is optional and only valid when `--tool opencode` (or `-T oc`) is selected.
- When `--variant` is provided for non-opencode tools, the program should return a clear error.
- When `--variant` is provided for opencode, pass `--variant <value>` to the `opencode` CLI.
- Preserve existing behavior for all other flags and tools.

## Non-Goals
- No automatic validation against the live list of variants.
- No changes to model alias resolution.
- No changes to the behavior of `claude` or `codex` tools.

## CLI UX
- `ralph -i 5 -d my-task -T opencode -v fast`
- `ralph -i 5 -d my-task -T oc --variant fast`

## Implementation Notes
- Add `variant: Option<String>` to `Cli` with `--variant` and `-v`.
- Extend tool creation to accept the optional variant and thread it into `OpenCodeTool`.
- Update the opencode command invocation to include `--variant` when present.
- Add a new error variant for invalid tool option usage.
- Update README usage/args section to document the new flag.

## Acceptance Criteria
- `ralph -T opencode -v <variant>` passes the flag to `opencode`.
- `ralph -T claude -v <variant>` fails fast with a clear error message.
- Existing commands without `--variant` behave unchanged.
