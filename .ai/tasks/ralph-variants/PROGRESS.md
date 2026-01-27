# Progress

## Completed (2025-01-22)

### Add `--variant` flag for OpenCode tool

Implemented the `--variant` flag (`-v` shorthand) that allows specifying OpenCode model variants. The flag only works with the `opencode` tool and is rejected for other tools.

**Changes made:**

1. **CLI arguments** (src/cli.rs):
   - Added `variant: Option<String>` field to `Cli` struct
   - Added `--variant` flag with `-v` shorthand
   - Implemented `Display` trait for `ToolChoice` enum (for error messages)

2. **Error handling** (src/error.rs):
   - Added `InvalidToolOption` error variant for invalid tool/flag combinations

3. **OpenCode tool** (src/tools/opencode.rs):
   - Updated `OpenCodeTool::new()` to accept `variant` parameter
   - Modified `run()` method to pass `--variant` to opencode command when present

4. **Tool factory** (src/tools/mod.rs):
   - Updated `create_tool()` factory function to accept and pass variant parameter

5. **Runner logic** (src/runner.rs):
   - Added validation to reject `--variant` flag for non-opencode tools
   - Updated tool creation to pass variant parameter

6. **Documentation** (README.md):
   - Updated usage example to include `--variant` flag
   - Added `--variant` to arguments table
   - Added example showing variant usage

**Testing:**
- All 26 existing tests pass
- Type checking passes with cargo check
- No new warnings introduced

**Usage examples:**
```bash
ralph -i 5 -d my-task -T opencode -v fast
ralph -i 10 -d my-task -T oc --variant fast
ralph -i 3 -d ./path/to/task -T cc -v fast  # Error: variant only for opencode
```

**Acceptance criteria met:**
- ✅ `ralph -T opencode -v <variant>` passes the flag to `opencode`
- ✅ `ralph -T claude -v <variant>` fails fast with clear error message
- ✅ Existing commands without `--variant` behave unchanged

