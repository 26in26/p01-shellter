use crate::{executor::ExecCommand, shell_state::ShellState};

pub fn run(_: &ExecCommand, state: &mut ShellState) {
    println!("{}", state.cwd.to_string_lossy());
}
