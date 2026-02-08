use std::process::{Command, Stdio};

use crate::executor::{ExecCommand, Executable, IoWiring};
use crate::shell_error::ShellError;
use crate::shell_state::ShellState;

pub struct External<'a> {
    cmd: &'a ExecCommand,
    child: Option<std::process::Child>,
    wires: Option<IoWiring>,
}

pub fn new(cmd: &ExecCommand) -> External<'_> {
    External {
        cmd: cmd,
        child: None,
        wires: None,
    }
}

impl<'a> Executable for External<'a> {
    fn spawn(&mut self, state: &mut ShellState) -> Result<(), ShellError> {
        let mut cmd = Command::new(&self.cmd.program);
        cmd.args(&self.cmd.args).current_dir(state.get_cwd());

        if self.wires.is_some() {
            cmd.stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped());
        } else {
            cmd.stdin(Stdio::inherit())
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit());
        };

        let mut child = cmd.spawn().map_err(|e| {
            ShellError::ExecutionError(format!("failed to execute {}: {}", self.cmd.program, e))
        })?;

        if self.wires.is_some() {
            self.wire_child(&mut child);
        }

        self.child = Some(child);

        Ok(())
    }

    fn wire(&mut self, wiring: IoWiring) -> Result<(), ShellError> {
        self.wires = Some(wiring);

        Ok(())
    }

    fn wait(&mut self) -> Result<i32, ShellError> {
        let child = self
            .child
            .as_mut()
            .ok_or_else(|| ShellError::ExecutionError("No command to wait for".to_string()))?;

        child
            .wait()
            .map(|status| status.code().unwrap_or(0))
            .map_err(|e| {
                ShellError::ExecutionError(format!(
                    "failed to wait for command {}: {}",
                    self.cmd.program, e
                ))
            })
    }
}

impl<'a> External<'a> {
    fn wire_child(&mut self, child: &mut std::process::Child) {
        if let Some(wiring) = self.wires.take() {
            let IoWiring {
                mut stdin,
                mut stdout,
                mut stderr,
            } = wiring;

            if let Some(mut child_stdin) = child.stdin.take() {
                std::thread::spawn(move || {
                    std::io::copy(&mut stdin, &mut child_stdin).ok();
                });
            }

            if let Some(mut child_stdout) = child.stdout.take() {
                std::thread::spawn(move || {
                    std::io::copy(&mut child_stdout, &mut stdout).ok();
                });
            }

            if let Some(mut child_stderr) = child.stderr.take() {
                std::thread::spawn(move || {
                    std::io::copy(&mut child_stderr, &mut stderr).ok();
                });
            }
        }
    }
}
