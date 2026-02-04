use crate::{executor::ExecCommand, shell_state::ShellState};

pub fn run(cmd: &ExecCommand, state: &mut ShellState) {
    println!("Exsiting...");
    state.exit = true;
}
