use std::process::Command;

use crate::executor::ExecCommand;
use crate::shell_error::ShellError;
use crate::shell_state::ShellState;

pub fn run(cmd: &ExecCommand, _state: &mut ShellState) -> Result<(), ShellError> {
    match Command::new(&cmd.program).args(&cmd.args).status() {
        Ok(_) => Ok(()),
        Err(e) => Err(ShellError::ExecutionError(format!(
            "Failed to execute command: {}",
            e
        ))),
    }
}
