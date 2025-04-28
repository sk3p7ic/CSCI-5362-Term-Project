use std::io::{self, Write};
use std::process::{Command, exit};
use std::ptr;
use std::os::unix::process::CommandExt;
use std::ffi::{CString, CStr};

fn expect<T>(ptr: Option<T>, msg: &str) {
    if ptr.is_none() {
        eprintln!("{}", msg);
        exit(1);
    }
}

fn buflen(s: &str) -> usize {
    s.len() + 1
}

fn prompt() {
    print!("mysh% ");
    io::stdout().flush().unwrap();
}

fn get_next_command(last_command: Option<&str>) -> Option<String> {
    let mut cmd_buf = String::new();
    prompt();

    if io::stdin().read_line(&mut cmd_buf).is_err() {
        return None;
    }

    let cmd_buf = cmd_buf.trim_end();

    if cmd_buf == "!!" {
        if let Some(last_cmd) = last_command {
            println!("mysh% {}", last_cmd);
            return Some(last_cmd.to_string());
        } else {
            eprintln!("No commands in history.");
            return None;
        }
    }

    Some(cmd_buf.to_string())
}

fn get_n_spaces(s: &str) -> usize {
    s.chars().filter(|&c| c == ' ').count()
}

fn tokenize(s: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut iter = s.split_whitespace().peekable();

    while let Some(token) = iter.next() {
        if token.starts_with('"') && token.ends_with('"') && token.len() > 1 {
            tokens.push(token[1..token.len()-1].to_string());
        } else {
            tokens.push(token.to_string());
        }
    }

    tokens
}

fn execute(args: Vec<String>) {
    let mut command = Command::new(&args[0]);
    let args_slice: Vec<&str> = args.iter().skip(1).map(|s| s.as_str()).collect();

    let do_wait = if args_slice.last() == Some(&"&") {
        command.arg("&");
        false 
    } else {
        true
    };

    let child = command.spawn().expect("Failed to start command");

    if do_wait {
        let _ = child.wait().expect("Command wasn't running");
    }
}

fn main() {
    let mut last_command: Option<String> = None;

    loop {
        if let Some(cmd_buf) = get_next_command(last_command.as_deref()) {
            last_command = Some(cmd_buf.clone());
            if cmd_buf == "exit" {
                break;
            }

            let n_args = get_n_spaces(&cmd_buf) + 1;
            let cmd_args = tokenize(&cmd_buf);
            execute(cmd_args);
        }
    }
}