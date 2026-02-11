use std::io::ErrorKind;
use std::process::{Command, Stdio};

use crate::executor::{ExecCommand, Executable, IoWiring, Stream};
use crate::shell_error::ShellError;
use crate::shell_state::ShellState;

pub struct External {
    program: String,
    command: Command,
    child: Option<std::process::Child>,
    wires: Option<IoWiring>,
}

pub fn new(cmd: &ExecCommand) -> External {
    let program = cmd.program.clone();
    let args = cmd.args.clone();

    let mut command = Command::new(&program);
    command.args(&args);
    command
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());

    External {
        program,
        command,
        child: None,
        wires: Some(IoWiring {
            stdin: Stream::Inherited,
            stdout: Stream::Inherited,
            stderr: Stream::Inherited,
        }),
    }
}

impl Executable for External {
    fn spawn(&mut self, _: &mut ShellState) -> Result<(), ShellError> {
        let mut child = self.command.spawn().map_err(|e| match e.kind() {
            ErrorKind::NotFound => ShellError::CommandNotFound(self.program.clone()),
            _ => ShellError::ExecutionError(format!("failed to execute {}: {}", self.program, e)),
        })?;

        self.wire_child(&mut child);

        self.child = Some(child);

        Ok(())
    }

    fn wire(&mut self, wiring: IoWiring) -> Result<(), ShellError> {
        match wiring.stdin {
            Stream::Piped(_) => {
                self.command.stdin(Stdio::piped());
            }
            Stream::Inherited => {
                self.command.stdin(Stdio::inherit());
            }
        }

        match wiring.stdout {
            Stream::Piped(_) => {
                self.command.stdout(Stdio::piped());
            }
            Stream::Inherited => {
                self.command.stdout(Stdio::inherit());
            }
        }

        match wiring.stderr {
            Stream::Piped(_) => {
                self.command.stderr(Stdio::piped());
            }
            Stream::Inherited => {
                self.command.stderr(Stdio::inherit());
            }
        }

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
                    self.program, e
                ))
            })
    }
}

impl External {
    fn wire_child(&mut self, child: &mut std::process::Child) {
        let Some(wiring) = self.wires.take() else {
            return;
        };

        if let Stream::Piped(mut stdin) = wiring.stdin {
            if let Some(mut child_stdin) = child.stdin.take() {
                std::thread::spawn(move || {
                    std::io::copy(&mut stdin, &mut child_stdin).ok();
                });
            }
        }

        if let Stream::Piped(mut stdout) = wiring.stdout {
            if let Some(mut child_stdout) = child.stdout.take() {
                std::thread::spawn(move || {
                    std::io::copy(&mut child_stdout, &mut stdout).ok();
                });
            }
        }

        if let Stream::Piped(mut stderr) = wiring.stderr {
            if let Some(mut child_stderr) = child.stderr.take() {
                std::thread::spawn(move || {
                    std::io::copy(&mut child_stderr, &mut stderr).ok();
                });
            }
        }
    }
}
