use crate::{executor::ExecCommand, shell_state::ShellState};

pub fn run(_cmd: &ExecCommand, state: &mut ShellState) {
    println!("Exsiting...");
    state.exit = true;
}
