use crate::executor::{ExecCommand, Executable, builtins, external};

pub fn make_simple_executable(cmd: &ExecCommand) -> Box<dyn Executable + '_> {
    match cmd.program.as_str() {
        "ls" => Box::new(builtins::ls::new(cmd)),
        "cd" => Box::new(builtins::cd::new(cmd)),
        "echo" => Box::new(builtins::echo::new(cmd)),
        "pwd" => Box::new(builtins::pwd::new()),
        "exit" => Box::new(builtins::exit::new()),
        _ => Box::new(external::new(cmd)),
    }
}
