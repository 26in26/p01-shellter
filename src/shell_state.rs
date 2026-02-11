use std::path::{self, absolute};

pub struct ShellState {
    last_status: i32,
    cwd: path::PathBuf,
    pub exit: bool,
}

impl ShellState {
    // get absolute path to the current working directory
    pub fn get_cwd(&self) -> &path::PathBuf {
        return &self.cwd;
    }

    pub fn set_cwd(&mut self, cwd: path::PathBuf) {
        let absolute_cwd = match absolute(cwd) {
            Ok(path) => path,
            Err(_) => return,
        };

        self.cwd = absolute_cwd;
    }

    pub fn get_last_status(&self) -> i32 {
        self.last_status
    }

    pub fn set_status(&mut self, status: i32) {
        self.last_status = status;
    }
}

pub fn new() -> ShellState {
    let absolute_cwd = absolute(path::PathBuf::from("."));

    let absolute_cwd = match absolute_cwd {
        Ok(path) => path,
        Err(_) => path::PathBuf::from("/"),
    };

    ShellState {
        last_status: 0,
        cwd: absolute_cwd,
        exit: false,
    }
}
