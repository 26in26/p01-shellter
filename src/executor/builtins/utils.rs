use crate::{executor::ExecCommand, shell_state::ShellState};
use std::{
    io::{Read, Write},
    path::PathBuf,
};

pub struct BuiltinWiring {
    pub stdin: Box<dyn Read + Send>,
    pub stdout: Box<dyn Write + Send>,
    pub stderr: Box<dyn Write + Send>,
}

pub fn get_target_path(cmd: &ExecCommand, state: &ShellState) -> PathBuf {
    match cmd.args.get(0) {
        Some(arg) => {
            let path = PathBuf::from(arg);
            if path.is_absolute() {
                path
            } else {
                state.get_cwd().join(path)
            }
        }
        None => state.get_cwd().clone(),
    }
}

pub fn get_default_builtin_wiring() -> BuiltinWiring {
    BuiltinWiring {
        stdin: (Box::new(std::io::stdin())),
        stdout: (Box::new(std::io::stdout())),
        stderr: (Box::new(std::io::stderr())),
    }
}
