use std::path::{self, absolute};

pub struct ShellState {
    // an aboslute path to the current working directory
    pub cwd: path::PathBuf,
    pub exit: bool,
}

pub fn new() -> ShellState {
    let absolute_cwd = absolute(path::PathBuf::from("."));

    let absolute_cwd = match absolute_cwd {
        Ok(path) => path,
        Err(_) => path::PathBuf::from("/"),
    };
    ShellState {
        cwd: absolute_cwd,
        exit: false,
    }
}
