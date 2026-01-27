use crate::error::{RalphError, Result};
use crate::task::TaskContext;
use crate::tools::Tool;
use std::io::{BufRead, BufReader};
use std::process::{Command, ExitStatus, Stdio};

pub struct OpenCodeTool {
    model: Option<String>,
    variant: Option<String>,
}

impl OpenCodeTool {
    pub fn new(model: Option<String>, variant: Option<String>) -> Self {
        Self { model, variant }
    }
}

impl Tool for OpenCodeTool {
    fn name(&self) -> &'static str {
        "opencode"
    }

    fn run(&self, _ctx: &TaskContext, prompt: &str) -> Result<(ExitStatus, String)> {
        let mut cmd = Command::new("opencode");
        cmd.arg("run").arg(prompt);

        if let Some(ref model) = self.model {
            cmd.args(["--model", model]);
        }

        if let Some(ref variant) = self.variant {
            cmd.args(["--variant", variant]);
        }

        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::inherit());

        let mut child = cmd.spawn().map_err(|e| RalphError::ToolFailed {
            tool: "opencode".into(),
            source: e,
        })?;

        let stdout = child.stdout.take().unwrap();
        let reader = BufReader::new(stdout);
        let mut output = String::new();

        for line in reader.lines() {
            let line = line.map_err(|e| RalphError::ToolFailed {
                tool: "opencode".into(),
                source: e,
            })?;
            println!("{}", line);
            output.push_str(&line);
            output.push('\n');
        }

        let status = child.wait().map_err(|e| RalphError::ToolFailed {
            tool: "opencode".into(),
            source: e,
        })?;

        Ok((status, output))
    }
}
