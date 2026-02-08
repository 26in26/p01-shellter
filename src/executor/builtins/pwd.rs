use super::utils;
use crate::executor::{Executable, IoWiring};
use crate::shell_error::ShellError;
use crate::shell_state::ShellState;

pub struct Pwd {
    wires: IoWiring,
}

pub fn new() -> Pwd {
    Pwd {
        wires: utils::get_default_wiring(),
    }
}

impl Executable for Pwd {
    fn spawn(&mut self, state: &mut ShellState) -> Result<(), ShellError> {
        let IoWiring {
            stdin: _,
            stdout,
            stderr: _,
        } = &mut self.wires;

        let cwd = state.get_cwd();
        writeln!(stdout, "{}", cwd.to_string_lossy()).map_err(|e| {
            ShellError::ExecutionError(format!("pwd: can't write to stdout: {}", e.to_string()))
        })?;

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
