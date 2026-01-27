use clap::{Parser, ValueEnum};
use std::fmt;

#[derive(Parser)]
#[command(
    name = "ralph",
    version,
    about = "Run AI coding tools iteratively on tasks",
    long_about = None
)]
pub struct Cli {
    /// Number of iterations to run
    #[arg(short = 'i', long)]
    pub iterations: u32,

    /// Directory path for task (use --task for task names in .ai/tasks/)
    #[arg(short = 'd', long)]
    pub dir: Option<String>,

    /// PRD file path or task name (resolves to .ai/tasks/{name}/PRD.md)
    #[arg(short = 't', long)]
    pub task: Option<String>,

    /// Tool to use for execution
    #[arg(short = 'T', long, value_enum)]
    pub tool: ToolChoice,

    /// Model name (optional, accepts aliases)
    #[arg(short = 'm', long)]
    pub model: Option<String>,

    /// OpenCode variant (only valid with --tool opencode)
    #[arg(short = 'v', long)]
    pub variant: Option<String>,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum ToolChoice {
    /// OpenCode AI tool
    #[value(name = "opencode", alias = "oc")]
    OpenCode,

    /// Claude Code CLI
    #[value(name = "claude", alias = "cc")]
    Claude,

    /// OpenAI Codex CLI
    #[value(name = "codex")]
    Codex,
}

impl fmt::Display for ToolChoice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ToolChoice::OpenCode => write!(f, "opencode"),
            ToolChoice::Claude => write!(f, "claude"),
            ToolChoice::Codex => write!(f, "codex"),
        }
    }
}
