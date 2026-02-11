use crate::executor::execute;
use crate::parser::parse;
use crate::shell_error::ShellError;
use crate::shell_state::ShellState;
use crate::utils::color::{BOLD, ITALIC, RESET};
use crate::utils::theme;

use std::io::{self, Write};

pub struct Shell {
    state: ShellState,
}

pub fn new(state: ShellState) -> Shell {
    Shell { state: state }
}

impl Shell {
    pub fn run(&mut self) {
        let mut line = String::new();

        loop {
            line.clear();
            self.print_prompt();
            if let Err(ShellError::IoError(_)) = self.read_command(&mut line) {
                continue;
            }

            let cmd = match parse(&line) {
                Ok(cmd) => cmd,
                Err(ShellError::ParseError(e)) => {
                    eprintln!("Parse error: {}", e);
                    continue;
                }
                _ => {
                    eprintln!("Unknown error");
                    continue;
                }
            };

            match execute(&cmd, &mut self.state) {
                Ok(status) => {
                    self.state.set_status(status);
                }
                Err(shell_error) => {
                    self.state.set_status(1);
                    match shell_error {
                        ShellError::CommandNotFound(cmd) => {
                            eprintln!("Command not found: {}", cmd);
                        }
                        ShellError::ExecutionError(msg) => {
                            eprintln!("Execution error: {}", msg);
                        }
                        ShellError::IoError(e) => {
                            eprintln!("IO error: {}", e);
                        }
                        _ => {}
                    }
                }
            }

            if self.state.exit {
                break;
            }
        }
    }

    fn read_command(&self, command: &mut String) -> Result<(), ShellError> {
        io::stdin().read_line(command)?;
        Ok(())
    }

    fn print_prompt(&self) {
        let status = if self.state.get_last_status() == 0 {
            theme::success()
        } else {
            theme::error()
        };

        print!(
            "{}{}rsh{} {}{}{}{} {}❯{} ",
            BOLD,
            theme::rust(),
            RESET,
            ITALIC,
            theme::path(),
            self.state.get_cwd().display(),
            RESET,
            status,
            RESET
        );
        io::stdout().flush().unwrap();
    }
}
