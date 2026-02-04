use crate::executor::builtins::utils::get_target_path;
use crate::shell_error::ShellError;
use crate::{executor::ExecCommand, shell_state::ShellState};
use std::fs;
use std::path::PathBuf;

pub fn run(cmd: &ExecCommand, state: &ShellState) -> Result<i32, ShellError> {
    let target: PathBuf = get_target_path(cmd, state);

    let entries = match fs::read_dir(&target) {
        Ok(entries) => entries,
        Err(err) => {
            eprintln!("ls: {}: {}", target.display(), err);
            return Ok(1);
        }
    };

    for entry in entries {
        let entry = match entry {
            Ok(entry) => entry,
            Err(err) => {
                eprintln!("ls: {}: {}", target.display(), err);
                continue;
            }
        };

        let name = entry.file_name();
        print!("{}  ", name.to_string_lossy());
    }

    println!();
    Ok(0)
}
