use std::io::{stdin, Write};
use std::io::stdout;
use std::process::Command;

fn main() {

    loop {
        // use the `>` character as the prompt
        // need to explicitly flush this to ensure it prints before read_line
        print!(">>> ");
        let _ = stdout().flush();

        let mut input = String::new();

        stdin().read_line(&mut input).unwrap();

        let mut parts = input.trim().split_whitespace();
        let command = parts.next().unwrap();
        let args = parts;


        let mut child = Command::new(command)
            .args(args)
            .spawn()
            .unwrap();

        // don't accept another command until this one completes
        let _ = child.wait();
    }
}
