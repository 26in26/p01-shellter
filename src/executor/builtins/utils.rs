use crate::{executor::ExecCommand, shell_state::ShellState};
use std::{
    io::{Read, Write},
    path::PathBuf,
};

pub struct BuiltinWiring {
    pub stdin: Option<Box<dyn Read + Send>>,
    pub stdout: Option<Box<dyn Write + Send>>,
    pub stderr: Option<Box<dyn Write + Send>>,
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
        stdin: Some(Box::new(std::io::stdin())),
        stdout: Some(Box::new(std::io::stdout())),
        stderr: Some(Box::new(std::io::stderr())),
    }
}
