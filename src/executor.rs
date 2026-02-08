use std::io::{Read, Write};
use std::os::windows;

use crate::parser::Command;
use crate::shell_error::ShellError;
use crate::shell_state::ShellState;

pub use crate::parser::SimpleCommand as ExecCommand;

pub mod builtins;
pub mod external;

pub struct IoWiring {
    pub stdin: Box<dyn Read + Send>,
    pub stdout: Box<dyn Write + Send>,
    pub stderr: Box<dyn Write + Send>,
}

pub trait Executable {
    fn spawn(&mut self, state: &mut ShellState) -> Result<(), ShellError>;

    fn wire(&mut self, wiring: IoWiring) -> Result<(), ShellError>;

    fn wait(&mut self) -> Result<i32, ShellError>;
}

pub fn execute(cmd: &Command, state: &mut ShellState) -> Result<(), ShellError> {
    match cmd {
        Command::Simple(cmd) => execute_simple(cmd, state)?,
    };

    Ok(())
}

fn execute_simple(cmd: &ExecCommand, state: &mut ShellState) -> Result<(), ShellError> {
    let wiring = IoWiring {
        stdin: Box::new(std::io::stdin()),
        stdout: Box::new(std::io::stdout()),
        stderr: Box::new(std::io::stderr()),
    };
    match cmd.program.as_str() {
        "ls" => {
            let mut cmd = builtins::ls::new(cmd);
            cmd.wire(wiring);
            cmd.spawn(state)?;
            cmd.wait()
        }
        "cd" => {
            let mut cmd = builtins::cd::new(cmd);
            cmd.wire(wiring);
            cmd.spawn(state)?;
            cmd.wait()
        }
        "pwd" => {
            let mut cmd = builtins::pwd::new();
            cmd.wire(wiring);
            cmd.spawn(state)?;
            cmd.wait()
        }
        "exit" => {
            let mut cmd = builtins::exit::new();
            cmd.wire(wiring);
            cmd.spawn(state)?;
            cmd.wait()
        }
        _ => {
            let mut cmd = external::new(cmd);
            if let Err(ShellError::ExecutionError(e)) = cmd.spawn(state) {
                eprintln!("{}", e);
            };
            cmd.wait()
        }
    };

    Ok(())
}
