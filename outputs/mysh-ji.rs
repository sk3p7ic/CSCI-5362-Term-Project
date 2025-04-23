use std::io::{self, Write};
use std::process::{Command, Stdio};
use std::ptr;
use std::ffi::{CString, CStr};
use std::os::unix::process::CommandExt;

fn expect<T>(ptr: Option<T>, msg: &str) {
    if ptr.is_none() {
        eprintln!("{}", msg);
        std::process::exit(1);
    }
}

fn buflen(str: &str) -> usize {
    str.len() + 1
}

fn prompt() {
    print!("mysh% ");
    io::stdout().flush().unwrap();
}

fn prompt_cmd(cmd: &str) {
    println!("mysh% {}", cmd);
}

fn get_next_command(last_command: Option<&str>) -> Option<String> {
    let mut cmd_buf = String::new();
    prompt();

    io::stdin().read_line(&mut cmd_buf).expect("Failed to read line");

    let cmd_buf = cmd_buf.trim_end(); // Remove trailing newline

    // If the user would like to run the last-run command
    if cmd_buf == "!!" {
        if let Some(last) = last_command {
            prompt_cmd(last);
            return Some(last.to_string());
        } else {
            println!("No commands in history.");
            return None;
        }
    }

    Some(cmd_buf.to_string())
}

fn get_n_spaces(s: &str) -> usize {
    s.matches(' ').count()
}

fn tokenize(s: &str, max_tokens: usize) -> Vec<Option<String>> {
    let mut tokens = Vec::with_capacity(max_tokens);
    let mut curr = s.split_whitespace();
    
    for token in curr {
        tokens.push(Some(token.to_string()));
    }

    tokens.push(None); // Null terminate the array
    tokens
}

fn execute(n_args: usize, args: Vec<Option<String>>) {
    expect(args.get(0), "Command argument cannot be null");

    let do_wait = match args.get(n_args - 2) {
        Some(Some(arg)) => arg != "&",
        _ => true,
    };

    let args: Vec<String> = args.iter().filter_map(|s| s.as_ref()).cloned().collect();
    let mut command = Command::new(&args[0]);

    command.args(&args[1..]);
    if !do_wait {
        command.spawn().expect("Error running child process");
    } else {
        let _ = command.output().expect("Error");
    }
}

fn main() {
    let mut last_command: Option<String> = None;

    loop {
        if let Some(cmd_buf) = get_next_command(last_command.as_deref()) {
            last_command = Some(cmd_buf.clone());

            // If the user would like to exit
            if cmd_buf == "exit" {
                break;
            }

            let n_args = get_n_spaces(&cmd_buf) + 1;
            let cmd_args = tokenize(&cmd_buf, n_args);
            execute(n_args, cmd_args);
        }
    }
}