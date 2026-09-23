use std::io::{self, Write};
use std::process::Command;

fn read_command() -> io::Result<Vec<String>> {
    print!("$ ");
    io::stdout().flush()?;

    let mut command = String::new();
    io::stdin().read_line(&mut command)?;

    let v = command
        .split_whitespace()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
    Ok(v)
}

fn main() {
    loop {
        let c = read_command().expect("la lecture peut echouer (ca arrive rarement)");
        let Some((command, args)) = c.split_first() else {
            continue;
        };

        if command == "exit" {
            break;
        }

        if command == "cd" {
            match args.first() {
                Some(dir) => {
                    if let Err(e) = std::env::set_current_dir(dir) {
                        println!("cd: {}", e)
                    };
                }
                None => println!("cd: no path"),
            }
            continue;
        }

        let command_result = Command::new(command).args(args).status();
        match command_result {
            Ok(success) => println!("{}", success),
            Err(e) => println!("{}", e),
        }
    }
}
