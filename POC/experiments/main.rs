use std::{
    io::{self, Write},
    process::Command,
};

const LINE: &'static str = "shellter:> ";

fn main() {
    let mut command = String::new();

    loop {
        print!("{}", LINE);
        // clean stdout buffer
        io::stdout().flush().unwrap();
        // clean command buffer
        command.clear();

        // read line from stdin
        if let Err(e) = io::stdin().read_line(&mut command) {
            eprintln!("error: {e}");
            break;
        };

        // trim command from whitespace and backslash n
        let input = command.trim();
        // check if input is empty
        if input.is_empty() {
            continue;
        }

        let mut parts = input.split_whitespace();
        let program = parts.next().unwrap();
        let args = parts;

        if program == "exit" {
            break;
        }

        // run command (fork() + exec() + wait())
        match Command::new(program).args(args).status() {
            Ok(_) => {}
            Err(e) => {
                eprintln!("error: {e}");
            }
        }
    }
}
