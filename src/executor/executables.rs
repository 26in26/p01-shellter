use crate::{executor::Executable, parser::Command};

pub mod pipe;
pub mod simple;

pub fn make_executable(command: &Command) -> Box<dyn Executable + '_> {
    match command {
        Command::Simple(cmd) => simple::make_simple_executable(&cmd),
        Command::Pipe(cmd) => pipe::make_pipeline_executable(&cmd),
        _ => todo!(),
    }
}
