use super::utils;
use crate::executor::{ExecCommand, Executable, IoWiring, Stream};
use crate::shell_error::ShellError;
use crate::shell_state::ShellState;

pub struct Echo<'a> {
    cmd: &'a ExecCommand,
    wires: utils::BuiltinWiring,
}

pub fn new(cmd: &'_ ExecCommand) -> Echo<'_> {
    Echo {
        wires: utils::get_default_builtin_wiring(),
        cmd: cmd,
    }
}

impl<'a> Executable for Echo<'a> {
    fn spawn(&mut self, _: &mut ShellState) -> Result<(), ShellError> {
        writeln!(self.wires.stdout, "{}", self.cmd.args.join(" ")).map_err(|e| {
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
