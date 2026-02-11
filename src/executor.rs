use std::io::{Read, Write};

use crate::parser::Command;
use crate::shell_error::ShellError;
use crate::shell_state::ShellState;

pub use crate::parser::SimpleCommand as ExecCommand;

pub mod builtins;
pub mod executables;
pub mod external;

pub enum Stream<T> {
    Piped(T),
    Inherited,
}

pub type Stdin = Stream<Box<dyn Read + Send>>;
pub type Stdout = Stream<Box<dyn Write + Send>>;
pub type Stderr = Stream<Box<dyn Write + Send>>;

pub struct IoWiring {
    pub stdin: Stdin,
    pub stdout: Stdout,
    pub stderr: Stderr,
}

pub trait Executable {
    fn spawn(&mut self, state: &mut ShellState) -> Result<(), ShellError>;

    fn wire(&mut self, wiring: IoWiring) -> Result<(), ShellError>;

    fn wait(&mut self) -> Result<i32, ShellError>;
}

pub fn execute(cmd: &Command, state: &mut ShellState) -> Result<(), ShellError> {
    let mut executable = executables::make_executable(cmd);
    executable.wire(IoWiring {
        stdin: Stream::Inherited,
        stdout: Stream::Inherited,
        stderr: Stream::Inherited,
    })?;
    executable.spawn(state)?;
    executable.wait()?;

    Ok(())
}
