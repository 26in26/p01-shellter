use crate::{executor::ExecCommand, shell_state::ShellState};
use std::path::PathBuf;

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
