#[allow(unused_imports)]
use std::io::{self, Write};

use std::{path::Path, process::Command};

fn main() {
    print!("$ ");
    io::stdout().flush().unwrap();

    // Wait for user input
    let stdin = io::stdin();
    loop {
        let mut input = String::new();
        stdin.read_line(&mut input).expect("failed to read input");
        let mut parts = input.trim().split(" ");

        match parts.next().unwrap() {
            "exit" => { break; }
            "echo" => { println!("{}", parts.collect::<Vec<&str>>().join(" ")); }
            "type" => { println!("{}", handle_types(parts.collect::<Vec<&str>>())) }
            "cd" => { handle_cd(parts.collect::<Vec<&str>>()) }
            x => { println!("{}", handle_none(x, parts.collect::<Vec<&str>>())) }
        }
        input.clear();

        print!("$ ");
        io::stdout().flush().unwrap();
    }
}


fn handle_types(input: Vec<&str>) -> String {
    let val = input[0];
    let path = std::env::var("PATH").unwrap();

    match val {
        "exit" | "echo" | "type" => {return String::from(format!("{val} is a shell builtin"))}

        x => { match path.split(":").map(|path| format!("{}/{}", path, x))
                                .find(|path| std::fs::metadata(path).is_ok()) {
            Some(path) => return format!("{}", path),
            _ => return format!("{}: not found", x),
            }
        }
    }
}

fn handle_none(x: &str, rest: Vec<&str>) -> String {
    let path = std::env::var("PATH").unwrap();
    let location = format!("{}/{}", std::env::current_dir().unwrap().display(), x);
    println!("{}", location);

    match path.split(":").map(|path| format!("{}/{}", path, x)).find(|path|std::fs::metadata(path).is_ok()) {
        Some(path) => { 
                let out = Command::new(path).args(rest).output().expect("failed to execute process");
                match std::str::from_utf8(&out.stdout) {
                    Ok(val) => format!("{}", val.to_string().trim()),
                    Err(_) => format!("{}", "Format of function must be string"),
                }
            }
        _ => { 
            if !std::fs::metadata(&location).is_ok() {
                return format!("{}: command not found", x.trim());
            } else {
                let out = Command::new(&location).args(rest).output().expect("failed to execute process");
                match std::str::from_utf8(&out.stdout) {
                    Ok(val) => format!("{}", val.to_string().trim()),
                    Err(_) => format!("{}", "Format of function must be string"),
                }
            }
        }
    }
}

fn handle_cd(args: Vec<&str>) {
    if args.len() != 1 {
        return;
    }
    let directory = args[0];
    let home = std::env::var("HOME").unwrap();
    let string_directory = directory.replace("~", home.as_str());
    

    let root = Path::new(&string_directory);
    if !std::env::set_current_dir(&root).is_ok() {
        println!("{}", format!("cd: {}: No such file or directory", directory));
    }
}