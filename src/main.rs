mod executor;
mod parser;
mod shell;
mod shell_error;
mod shell_state;

fn main() {
    let state = shell_state::new();
    let mut shell = shell::new(state);
    shell.run();
}
