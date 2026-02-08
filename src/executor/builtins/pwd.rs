use crate::executor::{ExecCommand, Executable, IoWiring};
use crate::shell_error::ShellError;
use crate::shell_state::ShellState;

pub struct Pwd {
    wires: Option<IoWiring>,
}

pub fn new() -> Pwd {
    Pwd { wires: None }
}

impl Executable for Pwd {
    fn spawn(&mut self, state: &mut ShellState) -> Result<(), ShellError> {
        let IoWiring {
            stdin: _,
            mut stdout,
            stderr: _,
        } = self
            .wires
            .take()
            .expect("wire() must be called before spawn()");

        let cwd = state.get_cwd();
        writeln!(stdout, "{}", cwd.to_string_lossy());

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
