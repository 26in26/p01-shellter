use std::env;

use crate::executor::builtins::utils::get_target_path;

use crate::executor::{ExecCommand, Executable, IoWiring};
use crate::shell_error::ShellError;
use crate::shell_state::ShellState;

pub struct Cd<'a> {
    cmd: &'a ExecCommand,
    wires: Option<IoWiring>,
}

pub fn new(cmd: &ExecCommand) -> Cd<'_> {
    Cd {
        cmd: cmd,
        wires: None,
    }
}

impl<'a> Executable for Cd<'a> {
    fn spawn(&mut self, state: &mut ShellState) -> Result<(), ShellError> {
        let IoWiring {
            stdin: _,
            stdout: _,
            mut stderr,
        } = self
            .wires
            .take()
            .expect("wire() must be called before spawn()");

        let target_path = get_target_path(self.cmd, state);

        match env::set_current_dir(&target_path) {
            Ok(_) => {
                state.set_cwd(target_path);
            }
            Err(err) => {
                writeln!(stderr, "cd: {}: {}", target_path.display(), err);
            }
        };

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
