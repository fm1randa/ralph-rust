use crate::cli::ToolChoice;
use crate::error::Result;
use crate::task::TaskContext;
use std::process::ExitStatus;

/// Trait for AI tool runners (Open/Closed Principle)
pub trait Tool {
    fn name(&self) -> &'static str;
    fn run(&self, ctx: &TaskContext, prompt: &str) -> Result<(ExitStatus, String)>;
}

mod claude;
mod codex;
mod opencode;

pub use claude::ClaudeTool;
pub use codex::CodexTool;
pub use opencode::OpenCodeTool;

/// Factory function for tool selection
pub fn create_tool(
    choice: &ToolChoice,
    model: Option<String>,
    variant: Option<String>,
) -> Box<dyn Tool> {
    match choice {
        ToolChoice::OpenCode => Box::new(OpenCodeTool::new(model, variant)),
        ToolChoice::Claude => Box::new(ClaudeTool::new(model)),
        ToolChoice::Codex => Box::new(CodexTool::new(model)),
    }
}
