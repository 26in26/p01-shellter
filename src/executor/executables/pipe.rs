use std::io::pipe;

use super::make_executable;
use crate::{
    executor::{Executable, IoWiring, Stdin, Stdout, Stream},
    parser::PipeCommand,
    shell_error::ShellError,
    shell_state::ShellState,
};

pub struct Pipeline<'a> {
    commands: Vec<Box<dyn Executable + 'a>>,
}

impl<'a> Executable for Pipeline<'a> {
    fn wire(&mut self, io: IoWiring) -> Result<(), ShellError> {
        if self.commands.is_empty() {
            return Ok(());
        }

        let mut pipeline_stdin = Some(io.stdin);
        let mut pipeline_stdout = Some(io.stdout);

        let n = self.commands.len();

        // We’ll build pipe pairs for connecting commands
        // Each pipe is (reader, writer)
        let mut pipes = Vec::with_capacity(n - 1);

        for _ in 0..n - 1 {
            let (r, w) = pipe().map_err(|e| ShellError::ExecutionError(e.to_string()))?;
            pipes.push((r, w));
        }

        for i in 0..n {
            let stdin: Stdin = if i == 0 {
                // First command reads from pipeline stdin
                pipeline_stdin.take().expect("pipeline stdin already taken")
            } else {
                // Read end of previous pipe
                Stream::Piped(Box::new(pipes[i - 1].0.try_clone()?))
            };

            let stdout: Stdout = if i == n - 1 {
                // Last command writes to pipeline stdout
                pipeline_stdout
                    .take()
                    .expect("pipeline stdout already taken")
            } else {
                // Write end of current pipe
                Stream::Piped(Box::new(pipes[i].1.try_clone()?))
            };

            let stderr = Stream::Inherited;

            self.commands[i].wire(IoWiring {
                stdin,
                stdout,
                stderr,
            })?;
        }

        Ok(())
    }

    fn spawn(&mut self, state: &mut ShellState) -> Result<(), ShellError> {
        for cmd in &mut self.commands {
            cmd.spawn(state)?;
        }
        Ok(())
    }

    fn wait(&mut self) -> Result<i32, ShellError> {
        for cmd in &mut self.commands {
            cmd.wait()?;
        }
        Ok(0)
    }
}

pub fn make_pipeline_executable(PipeCommand { stages }: &PipeCommand) -> Box<dyn Executable + '_> {
    let mut commands = Vec::new();

    for stage in stages {
        commands.push(make_executable(stage));
    }

    Box::new(Pipeline { commands })
}
