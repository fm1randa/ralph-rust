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
        cmd.args(["--dangerously-skip-permissions", "--no-session-persistence", "--include-partial-messages", "--output-format=stream-json", "--verbose", "-p", prompt]);

        if let Some(ref model) = self.model {
            cmd.args(["--model", model]);
        }

        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::inherit());

        let mut child = cmd.spawn().map_err(|e| RalphError::ToolFailed {
            tool: "claude".into(),
            source: e,
        })?;

        let mut printer = Command::new("npx")
            .arg("claude-pretty-printer")
            .stdin(Stdio::piped())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
            .map_err(|e| RalphError::ToolFailed {
                tool: "claude".into(),
                source: e,
            })?;

        let claude_stdout = child.stdout.take().unwrap();
        let printer_stdin = printer.stdin.take().unwrap();
        let reader = BufReader::new(claude_stdout);
        let mut writer = std::io::BufWriter::new(printer_stdin);
        let mut output = String::new();

        for line in reader.lines() {
            let line = line.map_err(|e| RalphError::ToolFailed {
                tool: "claude".into(),
                source: e,
            })?;
            use std::io::Write;
            writeln!(writer, "{}", line).map_err(|e| RalphError::ToolFailed {
                tool: "claude".into(),
                source: e,
            })?;
            output.push_str(&line);
            output.push('\n');
        }

        drop(writer);
        let _ = printer.wait();

        let status = child.wait().map_err(|e| RalphError::ToolFailed {
            tool: "claude".into(),
            source: e,
        })?;

        Ok((status, output))
    }
}
