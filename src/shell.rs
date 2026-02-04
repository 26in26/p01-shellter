use super::executor::execute;
use super::parser::parse;
use super::shell_error::ShellError;
use super::shell_state::ShellState;

use std::io::{self, Write};

pub struct Shell {
    state: ShellState,
}

pub fn new(state: ShellState) -> Shell {
    Shell { state: state }
}

impl Shell {
    fn print_prompt(&self) {
        print!("rsh ❯ ");
        io::stdout().flush().unwrap();
    }

    pub fn run(&mut self) {
        let mut line = String::new();

        loop {
            line.clear();
            self.print_prompt();
            if let Err(ShellError::IoError(_)) = self.read_command(&mut line) {
                continue;
            }
            let cmd = parse(&line).unwrap();
            execute(&cmd, &mut self.state).unwrap();

            if self.state.exit {
                break;
            }
        }
    }

    fn read_command(&self, command: &mut String) -> Result<(), ShellError> {
        io::stdin().read_line(command)?;
        Ok(())
    }
}
