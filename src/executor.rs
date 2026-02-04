use crate::parser::Command;
use crate::shell_error::ShellError;
use crate::shell_state::ShellState;

pub use crate::parser::SimpleCommand as ExecCommand;

pub mod builtins;
pub mod external;

pub fn execute(cmd: &Command, state: &mut ShellState) -> Result<(), ShellError> {
    match cmd {
        Command::Simple(cmd) => execute_simple(cmd, state)?,
    };

    Ok(())
}

fn execute_simple(cmd: &ExecCommand, state: &mut ShellState) -> Result<(), ShellError> {
    match cmd.program.as_str() {
        "ls" => {
            builtins::ls::run(cmd, state);
        }
        "cd" => {
            builtins::cd::run(cmd, state);
        }
        "pwd" => builtins::pwd::run(cmd, state),
        "exit" => builtins::exit::run(cmd, state),
        _ => {
            if let Err(ShellError::ExecutionError(e)) = external::run(cmd, state) {
                eprintln!("{}", e);
            };
        }
    };

    Ok(())
}
