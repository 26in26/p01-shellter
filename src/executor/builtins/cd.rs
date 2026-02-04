use std::env;

use crate::executor::builtins::utils::get_target_path;
use crate::shell_error::ShellError;
use crate::{executor::ExecCommand, shell_state::ShellState};

pub fn run(cmd: &ExecCommand, state: &mut ShellState) -> Result<i32, ShellError> {
    let target_path = get_target_path(cmd, state);

    match env::set_current_dir(&target_path) {
        Ok(_) => {
            state.set_cwd(target_path);
        }
        Err(err) => {
            eprintln!("cd: {}: {}", target_path.display(), err);
        }
    };

    Ok(0)
}
