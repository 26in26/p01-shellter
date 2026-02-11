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
    let parts = lex(line)?;

    if parts.is_empty() {
        return Err(ShellError::ParseError("can't parse string".to_string()));
    }

    // Check for pipes
    if parts.contains(&Token::Pipe) {
        return build_pipe_command(&parts);
    }

    Ok(build_simple_command(&parts))
}

fn build_simple_command(parts: &Vec<Token>) -> Command {
    let mut program = String::new();
    let mut args = Vec::new();

    parts.iter().for_each(|part| {
        if let Token::Word(word) = part {
            if program.is_empty() {
                program = word.to_string();
            } else {
                args.push(word.to_string());
            }
        }
    });

    Command::Simple(SimpleCommand { program, args })
}

fn build_pipe_command(parts: &Vec<Token>) -> Result<Command, ShellError> {
    let mut stages = Vec::new();
    let mut current_stage: Vec<&str> = Vec::new();

    for part in parts {
        match part {
            Token::Pipe => {
                if current_stage.is_empty() {
                    return Err(ShellError::ParseError("Empty pipe stage".to_string()));
                }
                stages.push(Command::Simple(SimpleCommand {
                    program: current_stage[0].to_string(),
                    args: current_stage[1..].iter().map(|s| s.to_string()).collect(),
                }));
                current_stage.clear();
            }
            Token::Word(word) => {
                current_stage.push(word);
            }
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

#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    Word(&'a str),
    Pipe,
}

pub fn lex(input: &'_ str) -> Result<Vec<Token<'_>>, ShellError> {
    let input = input.trim();
    let bytes = input.as_bytes();
    let mut tokens = Vec::new();

    let mut i = 0;

    while i < bytes.len() {
        match bytes[i] {
            b' ' => {
                i += 1;
            }

            b'|' => {
                tokens.push(Token::Pipe);
                i += 1;
            }

            b'"' => {
                // Quoted string
                let start = i + 1;
                i += 1;

                while i < bytes.len()
                    && !(bytes[i] == b'"'
                        && ((i + 1 < bytes.len() && bytes[i + 1] == b' ') || i == bytes.len() - 1))
                {
                    i += 1;
                }

                if i >= bytes.len() {
                    return Err(ShellError::ParseError("Unclosed quote".to_string()));
                }

                let slice = &input[start..i];
                tokens.push(Token::Word(slice));
                i += 1; // skip closing quote
            }

            _ => {
                // Normal word
                let start = i;

                while i < bytes.len() && bytes[i] != b' ' && bytes[i] != b'|' {
                    i += 1;
                }

                let slice = &input[start..i];
                tokens.push(Token::Word(slice));
            }
        }
    }

    Ok(tokens)
}
