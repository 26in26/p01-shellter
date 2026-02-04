use crate::{executor::ExecCommand, shell_state::ShellState};

pub fn run(_: &ExecCommand, state: &mut ShellState) {
    let cwd = state.get_cwd();
    println!("{}", cwd.to_string_lossy());
}
