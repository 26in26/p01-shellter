use super::shell_error::ShellError;

#[derive(Debug)]
pub struct SimpleCommand {
    pub program: String,
    pub args: Vec<String>,
}

#[derive(Debug)]
pub struct PipeCommand {
    pub stages: Vec<Command>,
}

#[derive(Debug)]
pub enum Command {
    Simple(SimpleCommand),
    Pipe(PipeCommand),
}

pub fn parse(line: &str) -> Result<Command, ShellError> {
    let parts: Vec<&str> = line.trim().split_whitespace().collect();
    if parts.is_empty() || parts[0].is_empty() {
        return Err(ShellError::ParseError("can't parse string".to_string()));
    }

    // Check for pipes
    if parts.contains(&"|") {
        let mut stages = Vec::new();
        let mut current_stage: Vec<&str> = Vec::new();

        for part in parts {
            if part == "|" {
                if current_stage.is_empty() {
                    return Err(ShellError::ParseError("empty pipe stage".to_string()));
                }
                stages.push(Command::Simple(SimpleCommand {
                    program: current_stage[0].to_string(),
                    args: current_stage[1..].iter().map(|s| s.to_string()).collect(),
                }));
                current_stage.clear();
            } else {
                current_stage.push(part);
            }
        }

        if current_stage.is_empty() {
            return Err(ShellError::ParseError("empty pipe stage".to_string()));
        }
        stages.push(Command::Simple(SimpleCommand {
            program: current_stage[0].to_string(),
            args: current_stage[1..].iter().map(|s| s.to_string()).collect(),
        }));

        return Ok(Command::Pipe(PipeCommand { stages }));
    }

    Ok(Command::Simple(SimpleCommand {
        program: parts[0].to_string(),
        args: parts[1..].iter().map(|s| s.to_string()).collect(),
    }))
}
