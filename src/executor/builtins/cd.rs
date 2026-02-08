use std::env;

use crate::executor::builtins::utils::{self, get_target_path};

use crate::executor::{ExecCommand, Executable, IoWiring};
use crate::shell_error::ShellError;
use crate::shell_state::ShellState;

pub struct Cd<'a> {
    cmd: &'a ExecCommand,
    wires: IoWiring,
}

pub fn new(cmd: &ExecCommand) -> Cd<'_> {
    Cd {
        cmd: cmd,
        wires: utils::get_default_wiring(),
    }
}

impl<'a> Executable for Cd<'a> {
    fn spawn(&mut self, state: &mut ShellState) -> Result<(), ShellError> {
        let IoWiring {
            stdin: _,
            stdout: _,
            stderr,
        } = &mut self.wires;

        let target_path = get_target_path(self.cmd, state);

        match env::set_current_dir(&target_path) {
            Ok(_) => {
                state.set_cwd(target_path);
            }
            Err(err) => {
                writeln!(stderr, "cd: {}: {}", target_path.display(), err).map_err(|e| {
                    ShellError::ExecutionError(format!(
                        "cd: can't write to stderr: {}",
                        e.to_string()
                    ))
                })?;
            }
        };

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
