use crate::cli::{Cli, ToolChoice};
use crate::error::{RalphError, Result};
use crate::models::{default_model, resolve_model};
use crate::task::TaskContext;
use crate::tools::create_tool;
use std::io::{self, Write};

const COMPLETION_MARKER: &str = "<promise>COMPLETE</promise>";

pub fn run(cli: Cli) -> Result<()> {
    // Validation: Check if both flags are provided
    if cli.dir.is_some() && cli.task.is_some() {
        eprintln!("Warning: Both --dir and --task provided. Ignoring --dir.");
    }

    // Validation: Check if neither flag is provided
    if cli.dir.is_none() && cli.task.is_none() {
        return Err(RalphError::NoInputProvided);
    }

    // Dispatch: Prefer --task over --dir when both are present
    let ctx = if let Some(ref task) = cli.task {
        TaskContext::discover_from_prd(task)?
    } else {
        TaskContext::discover(cli.dir.as_ref().unwrap())?
    };

    if cli.variant.is_some() && !matches!(cli.tool, ToolChoice::OpenCode) {
        return Err(RalphError::InvalidToolOption {
            tool: cli.tool.to_string(),
        });
    }

    let model = cli
        .model
        .map(|m| resolve_model(&cli.tool, &m))
        .or_else(|| default_model(&cli.tool));

    let tool = create_tool(&cli.tool, model.clone(), cli.variant.clone());

    println!("Found files:");
    println!("  Task dir:      {}", ctx.dir.display());
    println!("  PRD file:      {}", ctx.prd_file.display());
    println!("  PROGRESS file: {}", ctx.progress_file.display());
    println!("  Tool:          {}", tool.name());
    if let Some(ref m) = model {
        println!("  Model:         {}", m);
    }
    if let Some(ref v) = cli.variant {
        println!("  Variant:       {}", v);
    }
    println!();

    if !confirm_proceed()? {
        return Err(RalphError::UserCancelled);
    }

    let prompt = build_prompt(&ctx, &cli.tool);

    for i in 1..=cli.iterations {
        println!("\n--- Iteration {}/{} ---\n", i, cli.iterations);

        let (status, output) = tool.run(&ctx, &prompt)?;

        if !status.success() {
            eprintln!("Tool exited with non-zero status: {:?}", status.code());
        }

        if output.contains(COMPLETION_MARKER) {
            println!("\nPRD complete after {} iteration(s).", i);
            return Ok(());
        }
    }

    println!("\nCompleted {} iteration(s).", cli.iterations);
    Ok(())
}

fn build_prompt(ctx: &TaskContext, tool: &ToolChoice) -> String {
    let attachments = match tool {
        ToolChoice::Codex => format!(
            "PRD file: {}. PROGRESS file: {}.",
            ctx.prd_file.display(),
            ctx.progress_file.display()
        ),
        _ => format!(
            "@{} @{} @~/.agents/AGENTS.md",
            ctx.progress_file.display(),
            ctx.prd_file.display()
        ),
    };

    let rules_line = match tool {
        ToolChoice::Codex => "YOU **MUST** OBEY RULES SPECIFIED IN ~/.agents/AGENTS.md.",
        _ => "YOU **MUST** OBEY RULES SPECIFIED IN @~/.agents/AGENTS.md.",
    };

    format!(
        "{} \
        1. Find the highest-priority task and implement it. \
        2. Run your tests and type checks. \
        3. Update the PRD with what was done. \
        4. Append your progress to PROGRESS.md. \
        5. Commit your changes. \
        ONLY WORK ON A SINGLE TASK. \
        {} \
        If the PRD is complete, output <promise>COMPLETE</promise>.",
        attachments, rules_line
    )
}

fn confirm_proceed() -> Result<bool> {
    print!("Proceed with these files? [y/N] ");
    io::stdout().flush().ok();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .map_err(|e| RalphError::ToolFailed {
            tool: "stdin".into(),
            source: e,
        })?;

    Ok(input.trim().eq_ignore_ascii_case("y"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn mock_task_context() -> TaskContext {
        TaskContext {
            dir: PathBuf::from("/test/dir"),
            prd_file: PathBuf::from("/test/dir/PRD.md"),
            progress_file: PathBuf::from("/test/dir/PROGRESS.md"),
        }
    }

    #[test]
    fn test_build_prompt_opencode() {
        let ctx = mock_task_context();
        let prompt = build_prompt(&ctx, &ToolChoice::OpenCode);

        assert!(prompt.contains("@/test/dir/PROGRESS.md"));
        assert!(prompt.contains("@/test/dir/PRD.md"));
        assert!(prompt.contains("@~/.agents/AGENTS.md"));
        assert!(prompt.contains("highest-priority task"));
        assert!(prompt.contains("<promise>COMPLETE</promise>"));
    }

    #[test]
    fn test_build_prompt_claude() {
        let ctx = mock_task_context();
        let prompt = build_prompt(&ctx, &ToolChoice::Claude);

        assert!(prompt.contains("@/test/dir/PROGRESS.md"));
        assert!(prompt.contains("@/test/dir/PRD.md"));
        assert!(prompt.contains("@~/.agents/AGENTS.md"));
    }

    #[test]
    fn test_build_prompt_codex() {
        let ctx = mock_task_context();
        let prompt = build_prompt(&ctx, &ToolChoice::Codex);

        assert!(prompt.contains("PRD file: /test/dir/PRD.md"));
        assert!(prompt.contains("PROGRESS file: /test/dir/PROGRESS.md"));
        assert!(prompt.contains("~/.agents/AGENTS.md"));
        assert!(!prompt.contains("@~/.agents/AGENTS.md"));
    }

    #[test]
    fn test_completion_marker() {
        assert_eq!(COMPLETION_MARKER, "<promise>COMPLETE</promise>");
    }
}
