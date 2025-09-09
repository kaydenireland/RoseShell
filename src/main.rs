use std::env;
use std::io::{stdin, Write};
use std::io::stdout;
use std::path::Path;
use std::process::Command;

// https://www.joshmcguigan.com/blog/build-your-own-shell-rust/

fn main() {

    loop {
        // use the `>` character as the prompt
        // need to explicitly flush this to ensure it prints before read_line
        print!(">>> ");
        let _ = stdout().flush();

        let mut input = String::new();

        stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        let mut parts = input.split_whitespace();
        let command = parts.next().unwrap();
        let args = parts;


        match command {
            "cd" => {
                let new_dir = args.peekable().peek().map_or("/", |x| *x);
                let root = Path::new(new_dir);
                if let Err(e) = env::set_current_dir(&root) {
                    eprintln!("{}", e);
                }
            },
            "dir" => {
                let child = Command::new("cmd")
                    .args(&["/C", "dir"])
                    .spawn();
                match child {
                    Ok(mut child) => { let _ = child.wait(); }
                    Err(e) => eprintln!("{}", e),
                }
            },
            "exit" => return,
            command => {
                let child = Command::new(command)
                    .args(args)
                    .spawn();

                // gracefully handle malformed user input
                match child {
                    Ok(mut child) => { let _ = child.wait(); },
                    Err(e) => eprintln!("{}", e),
                };
            }
        }
    }
}
