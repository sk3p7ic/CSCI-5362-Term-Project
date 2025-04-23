use std::io::{self, Write};
use std::ffi::CString;
use std::ptr;
use std::process::{Command, exit};
use std::os::unix::process::CommandExt;
use std::mem;
use std::ptr::null_mut;

/// Ensures that a given pointer `ptr` is not null. Prints `msg` and associated
/// error information, if any, and exits with `EXIT_FAILURE`.
macro_rules! expect {
    ($ptr:expr, $msg:expr) => {
        if $ptr.is_null() {
            eprintln!("{}", $msg);
            exit(1);
        }
    };
}

/// Calculates the size of a buffer, accounting for the null terminator (`\0`)
macro_rules! buflen {
    ($str:expr) => {
        $str.len() + 1
    };
}

/// Displays the shell prompt
macro_rules! prompt {
    () => {
        print!("mysh% ");
        io::stdout().flush().unwrap();
    };
}

/// Displays the shell prompt with a command
macro_rules! prompt_cmd {
    ($cmd:expr) => {
        println!("mysh% {}", $cmd);
    };
}

/// Gets the next command from the user.
fn get_next_command(last_command: Option<&str>) -> Option<String> {
    let mut cmd_buf_size = 16; // Initial size of the buffer
    let mut cmd_buf: Vec<u8> = Vec::with_capacity(cmd_buf_size);

    prompt!();

    // Get the input from the user
    // This is done char-by-char to allow for the dynamic buffer size
    let mut idx = 0;
    loop {
        let mut buf = [0u8; 1];
        if io::stdin().read_exact(&mut buf).is_err() {
            break;
        }
        if buf[0] == b'\n' {
            break; // End on newline
        }

        // If the input buffer needs to be bigger
        if idx + 1 >= cmd_buf_size {
            cmd_buf_size *= 2;
            cmd_buf.resize(cmd_buf_size, 0);
        }

        cmd_buf[idx] = buf[0]; // Save this character to the buffer
        idx += 1;
    }
    cmd_buf.truncate(idx); // Keep only the valid input

    // Ensure that the command string is null-terminated and convert to String
    let command_str = String::from_utf8(cmd_buf).ok()?;
    
    // If the user would like to run the last-run command
    if command_str == "!!" {
        if let Some(last) = last_command {
            prompt_cmd!(last);
            return Some(last.to_string());
        } else {
            println!("No commands in history.");
            return None;
        }
    }

    Some(command_str)
}

/// Gets the number of spaces found in a string.
fn get_n_spaces(str: &str) -> usize {
    str.matches(' ').count()
}

/// Tokenizes a given string into an array of tokens. Quoted strings are
/// parsed as a single string literal token.
fn tokenize(str: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;

    for c in str.chars() {
        match c {
            '"' => in_quotes = !in_quotes,
            ' ' if !in_quotes => {
                if !current.is_empty() {
                    tokens.push(current.clone());
                    current.clear();
                }
            }
            _ => current.push(c),
        }
    }
    
    if !current.is_empty() {
        tokens.push(current);
    }

    tokens
}

/// Executes a given command. Supports asynchronously running commands.
fn execute(n_args: usize, args: Vec<String>) {
    if n_args == 0 {
        return;
    }

    let async_mode = *args.last().unwrap() != "&";
    let command = &args[0];
    let args: Vec<CString> = args.iter().map(|arg| CString::new(arg.clone()).unwrap()).collect();

    unsafe {
        let pid = libc::fork();

        match pid {
            -1 => {
                eprintln!("Error running child process");
                exit(1);
            },
            0 => {
                // Convert to raw pointers for execvp
                let argv: Vec<*const libc::c_char> = args.iter().map(|s| s.as_ptr()).collect();
                execvp(command.as_ptr() as *const _, argv.as_ptr() as *const _);
                eprintln!("Error");
                exit(1);
            },
            _ => {
                if async_mode {
                    libc::waitpid(pid, ptr::null_mut(), 0);
                }
            }
        }
    }
}

fn main() {
    let mut last_command: Option<String> = None;

    loop {
        let cmd_buf = get_next_command(last_command.as_deref());

        if let Some(cmd) = cmd_buf {
            last_command = Some(cmd.clone());

            if cmd == "exit" {
                return;
            }

            let n_args = get_n_spaces(&cmd) + 1;
            let cmd_args = tokenize(&cmd);

            execute(n_args, cmd_args);
        }
    }
}