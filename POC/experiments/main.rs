use std::{
    io::{self, Write},
    process::Command,
};

const LINE: &'static str = "shellter:> ";

fn main() {
    let mut command = String::new();

    loop {
        print!("{}", LINE);
        io::stdout().flush().unwrap();
        command.clear();

        if let Err(e) = io::stdin().read_line(&mut command) {
            eprintln!("error: {e}");
            break;
        };

        let input = command.trim();
        if input.is_empty() {
            continue;
        }

        let mut parts = input.split_whitespace();
        let program = parts.next().unwrap();
        let args = parts;

        if program == "exit" {
            break;
        }

        match Command::new(program).args(args).status() {
            Ok(_) => {}
            Err(e) => {
                eprintln!("error: {e}");
            }
        }
    }
}
