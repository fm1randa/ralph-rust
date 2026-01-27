use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RalphError {
    #[error("Task directory not found: {path}")]
    TaskNotFound { path: PathBuf },

    #[error("No PRD file found in {dir}")]
    NoPrdFile { dir: PathBuf },

    #[error("Tool execution failed: {tool}")]
    ToolFailed {
        tool: String,
        #[source]
        source: std::io::Error,
    },

    #[error("User cancelled operation")]
    UserCancelled,

    #[error("Either --dir (-d) or --task (-t) must be provided")]
    NoInputProvided,

    #[error("The '--variant' flag is only supported for opencode tool, not for '{tool}'")]
    InvalidToolOption { tool: String },
}

pub type Result<T> = std::result::Result<T, RalphError>;
