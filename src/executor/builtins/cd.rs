use std::env;

use crate::executor::builtins::utils::{self, get_target_path};

use crate::executor::{ExecCommand, Executable, IoWiring, Stream};
use crate::shell_error::ShellError;
use crate::shell_state::ShellState;

pub struct Cd<'a> {
    cmd: &'a ExecCommand,
    wires: utils::BuiltinWiring,
}

pub fn new(cmd: &ExecCommand) -> Cd<'_> {
    Cd {
        cmd: cmd,
        wires: utils::get_default_builtin_wiring(),
    }
}

impl<'a> Executable for Cd<'a> {
    fn spawn(&mut self, state: &mut ShellState) -> Result<(), ShellError> {
        let target_path = get_target_path(self.cmd, state);

        match env::set_current_dir(&target_path) {
            Ok(_) => {
                state.set_cwd(target_path);
            }
            Err(err) => {
                writeln!(self.wires.stderr, "cd: {}: {}", target_path.display(), err).map_err(
                    |e| {
                        ShellError::ExecutionError(format!(
                            "cd: can't write to stderr: {}",
                            e.to_string()
                        ))
                    },
                )?;
            }
        };

        Ok(())
    }

    fn wire(&mut self, wiring: IoWiring) -> Result<(), ShellError> {
        let default_wiring = utils::get_default_builtin_wiring();

        self.wires.stdin = match wiring.stdin {
            Stream::Piped(stdin) => stdin,
            _ => default_wiring.stdin,
        };

        self.wires.stdout = match wiring.stdout {
            Stream::Piped(stdout) => stdout,
            _ => default_wiring.stdout,
        };

        self.wires.stderr = match wiring.stderr {
            Stream::Piped(stderr) => stderr,
            _ => default_wiring.stderr,
        };

        Ok(())
    }

    fn wait(&mut self) -> Result<i32, ShellError> {
        Ok(0)
    }
}
