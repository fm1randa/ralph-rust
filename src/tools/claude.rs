use crate::error::{RalphError, Result};
use crate::task::TaskContext;
use crate::tools::Tool;
use std::io::{BufRead, BufReader};
use std::process::{Command, ExitStatus, Stdio};

pub struct ClaudeTool {
    model: Option<String>,
}

impl ClaudeTool {
    pub fn new(model: Option<String>) -> Self {
        Self { model }
    }
}

impl Tool for ClaudeTool {
    fn name(&self) -> &'static str {
        "claude"
    }

    fn run(&self, _ctx: &TaskContext, prompt: &str) -> Result<(ExitStatus, String)> {
        let mut cmd = Command::new("claude");
        cmd.args(["--print", "--dangerously-skip-permissions", "-p", prompt]);

        if let Some(ref model) = self.model {
            cmd.args(["--model", model]);
        }

        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::inherit());

        let mut child = cmd.spawn().map_err(|e| RalphError::ToolFailed {
            tool: "claude".into(),
            source: e,
        })?;

        let stdout = child.stdout.take().unwrap();
        let reader = BufReader::new(stdout);
        let mut output = String::new();

        for line in reader.lines() {
            let line = line.map_err(|e| RalphError::ToolFailed {
                tool: "claude".into(),
                source: e,
            })?;
            println!("{}", line);
            output.push_str(&line);
            output.push('\n');
        }

        let status = child.wait().map_err(|e| RalphError::ToolFailed {
            tool: "claude".into(),
            source: e,
        })?;

        Ok((status, output))
    }
}
