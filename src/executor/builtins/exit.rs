use crate::executor::builtins::utils;
use crate::executor::{Executable, IoWiring, Stream};
use crate::shell_error::ShellError;
use crate::shell_state::ShellState;

pub struct Exit {
    wires: utils::BuiltinWiring,
}

pub fn new() -> Exit {
    Exit {
        wires: utils::get_default_builtin_wiring(),
    }
}

impl Executable for Exit {
    fn spawn(&mut self, state: &mut ShellState) -> Result<(), ShellError> {
        writeln!(self.wires.stdout, "Exsiting...").map_err(|e| {
            ShellError::ExecutionError(format!("exit: can't write to stdout: {}", e.to_string()))
        })?;
        state.exit = true;

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
