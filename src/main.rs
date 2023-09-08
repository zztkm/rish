use std::io::{self, Write};
use std::env::current_dir;
use std::process::{exit, Stdio};
use crate::{exec::Executor}

fn read_line(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("read_line failed");
    buffer
}

fn generate_prompt() -> io::Result<String> {
    let cwd = current_dir()?;
    Ok(format!("{} > ", cwd.display()))
}

fn exec(commando: Vec<&str>) -> io::Result<()> {
    let program = commando[0];
    let args = &commando[1..];
    let command = std::process::Command::new(program)
        .args(args)
        .stdout(Stdio::piped())
        .spawn();
    let child = match command {
        Ok(child) => child,
        Err(_) => {
            println!("rish: could not execute command: {}", program);
            return Ok(())
        }
    };
    let output = child.wait_with_output()?;
    // write to stdout
    io::stdout().write_all(&output.stdout)?;
    Ok(())
}

fn parse_line(buf: &str) -> Vec<&str> {
    let mut parts = buf.trim_end().split_whitespace();
    let program = match parts.next() {
        Some(program) => program,
        None => return vec![],
    };
    let args = parts;
    let mut command = vec![program];
    command.extend(args);
    command
}

fn main() {
    let exit_code = loop {
        let result = generate_prompt();
        let prompt = match result {
            Ok(prompt) => prompt,
            Err(_) => {
                println!("Error: could not generate prompt");
                break 1;
            }
        };
        let buf = read_line(&prompt);
        let parsed_line = parse_line(&buf);

        if parsed_line.len() == 0 {
            println!();
            continue;
        }

        match parsed_line[0] {
            "exit" => break 0,
            _ => {()},
        };
        let result = exec(parsed_line);
        match result {
            Ok(_) => {()},
            Err(_) => {
                println!("Error: could not execute command");
                break 1;
            }
        };
    };
    exit(exit_code);
}
