use super::utils;
use crate::executor::{Executable, IoWiring, Stream};
use crate::shell_error::ShellError;
use crate::shell_state::ShellState;

pub struct Pwd {
    wires: utils::BuiltinWiring,
}

pub fn new() -> Pwd {
    Pwd {
        wires: utils::get_default_builtin_wiring(),
    }
}

impl Executable for Pwd {
    fn spawn(&mut self, state: &mut ShellState) -> Result<(), ShellError> {
        let cwd = state.get_cwd();
        writeln!(self.wires.stdout, "{}", cwd.to_string_lossy()).map_err(|e| {
            ShellError::ExecutionError(format!("pwd: can't write to stdout: {}", e.to_string()))
        })?;

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
