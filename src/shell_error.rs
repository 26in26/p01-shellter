use std::io;

#[derive(Debug)]
pub enum ShellError {
    IoError(io::Error),
    ParseError(String),
    ExecutionError(String),
    CommandNotFound(String),
}

impl From<io::Error> for ShellError {
    fn from(err: io::Error) -> Self {
        ShellError::IoError(err)
    }
}
