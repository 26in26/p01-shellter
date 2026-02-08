use crate::executor::{Executable, IoWiring};
use crate::shell_error::ShellError;
use crate::shell_state::ShellState;

pub struct Exit {
    wires: Option<IoWiring>,
}

pub fn new() -> Exit {
    Exit { wires: None }
}

impl Executable for Exit {
    fn spawn(&mut self, state: &mut ShellState) -> Result<(), ShellError> {
        let IoWiring {
            stdin: _,
            mut stdout,
            stderr: _,
        } = self
            .wires
            .take()
            .expect("wire() must be called before spawn()");

        writeln!(stdout, "Exsiting...");
        state.exit = true;

        Ok(())
    }

    fn wire(&mut self, wiring: crate::executor::IoWiring) -> Result<(), ShellError> {
        self.wires = Some(wiring);
        Ok(())
    }

    fn wait(&mut self) -> Result<i32, ShellError> {
        Ok(0)
    }
}
