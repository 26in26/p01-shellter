use crate::executor::builtins::utils;
use crate::executor::{Executable, IoWiring};
use crate::shell_error::ShellError;
use crate::shell_state::ShellState;

pub struct Exit {
    wires: IoWiring,
}

pub fn new() -> Exit {
    Exit {
        wires: utils::get_default_wiring(),
    }
}

impl Executable for Exit {
    fn spawn(&mut self, state: &mut ShellState) -> Result<(), ShellError> {
        let IoWiring {
            stdin: _,
            stdout,
            stderr: _,
        } = &mut self.wires;

        writeln!(stdout, "Exsiting...").map_err(|e| {
            ShellError::ExecutionError(format!("exit: can't write to stdout: {}", e.to_string()))
        })?;
        state.exit = true;

        Ok(())
    }

    fn wire(&mut self, wiring: crate::executor::IoWiring) -> Result<(), ShellError> {
        self.wires = wiring;
        Ok(())
    }

    fn wait(&mut self) -> Result<i32, ShellError> {
        Ok(0)
    }
}
