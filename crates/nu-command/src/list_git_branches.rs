// Note: this is a temporary command that later will be converted into a pipeline

use std::process::Command as ProcessCommand;
use std::process::Stdio;

use nu_protocol::ast::Call;
use nu_protocol::engine::{Command, EvaluationContext};
use nu_protocol::{Signature, Value};

pub struct ListGitBranches;

//NOTE: this is not a real implementation :D. It's just a simple one to test with until we port the real one.
impl Command for ListGitBranches {
    fn name(&self) -> &str {
        "list-git-branches"
    }

    fn usage(&self) -> &str {
        "List the git branches of the current directory."
    }

    fn signature(&self) -> nu_protocol::Signature {
        Signature::build("list-git-branches")
    }

    fn run(
        &self,
        _context: &EvaluationContext,
        call: &Call,
        _input: Value,
    ) -> Result<nu_protocol::Value, nu_protocol::ShellError> {
        let list_branches = ProcessCommand::new("git")
            .arg("branch")
            .stdout(Stdio::piped())
            .spawn();

        if let Ok(child) = list_branches {
            if let Ok(output) = child.wait_with_output() {
                let val = output.stdout;

                let s = String::from_utf8_lossy(&val).to_string();

                let lines: Vec<_> = s
                    .lines()
                    .filter_map(|x| {
                        if x.starts_with("* ") {
                            None
                        } else {
                            Some(x.trim())
                        }
                    })
                    .map(|x| Value::String {
                        val: x.into(),
                        span: call.head,
                    })
                    .collect();

                Ok(Value::List {
                    vals: lines,
                    span: call.head,
                })
            } else {
                Ok(Value::Nothing { span: call.head })
            }
        } else {
            Ok(Value::Nothing { span: call.head })
        }
    }
}
