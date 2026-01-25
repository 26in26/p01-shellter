use super::shell_error::ShellError;

#[derive(Debug)]
pub struct SimpleCommand {
    pub program: String,
    pub args: Vec<String>,
}

#[derive(Debug)]
pub enum Command {
    Simple(SimpleCommand),
}

pub fn parse(line: &str) -> Result<Command, ShellError> {
    let parts: Vec<&str> = line.trim().split_whitespace().collect();
    if parts.is_empty() || parts[0].is_empty() {
        return Err(ShellError::ParseError("can't parse string".to_string()));
    }

    return Ok(Command::Simple(SimpleCommand {
        program: parts[0].to_string(),
        args: parts[1..].iter().map(|s| s.to_string()).collect(),
    }));
}
