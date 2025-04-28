use std::ffi::{CString, c_void};
use std::io::{self, Write};
use std::mem;
use std::ptr;
use std::os::unix::process::CommandExt;
use std::process::{Command, exit};
use std::ptr::null_mut;
use std::os::raw::c_char;

fn expect<T>(ptr: *const T, msg: &str) {
    if ptr.is_null() {
        eprintln!("{}", msg);
        std::process::exit(1);
    }
}

fn buflen(str_ptr: *const c_char) -> usize {
    unsafe { libc::strlen(str_ptr) + 1 }
}

fn prompt() {
    print!("mysh%% ");
    io::stdout().flush().unwrap();
}

fn prompt_cmd(cmd: &str) {
    println!("mysh%% {}", cmd);
}

fn get_next_command(last_command: Option<&str>) -> Option<String> {
    let mut cmd_buf = Vec::with_capacity(16);
    let mut idx = 0;

    prompt();

    while let Some(c) = getc() {
        if c == '\n' {
            break;
        }
        if idx + 1 >= cmd_buf.capacity() {
            cmd_buf.reserve(cmd_buf.capacity());
        }
        cmd_buf.push(c);
        idx += 1;
    }
    cmd_buf.push('\0'); // Null terminate

    let cmd_str = unsafe { CString::from_vec_unchecked(cmd_buf).into_string().unwrap() };

    if cmd_str == "!!" {
        if let Some(last_cmd) = last_command {
            let command_buf = CString::new(last_cmd).expect("Could not create CString from last command");
            prompt_cmd(command_buf.to_str().unwrap());
            return Some(last_cmd.to_string());
        } else {
            println!("No commands in history.");
            return None;
        }
    }

    Some(cmd_str)
}

fn get_n_spaces(s: &str) -> usize {
    s.chars().filter(|&c| c == ' ').count()
}

fn tokenize(input: &str, strs: &mut Vec<*mut c_char>) -> usize {
    let to_parse = CString::new(input).expect("Could not create CString");
    let to_parse_ptr = to_parse.as_ptr() as *mut c_char;

    let mut curr = to_parse_ptr;
    let mut idx = 0;

    // Parse tokens
    unsafe {
        while *curr != b'\0' {
            while *curr == b' ' {
                curr = curr.add(1);
            }
            if *curr == b'\0' { break; }

            let token_start = curr;
            let is_quoted = *curr == b'"';

            if is_quoted {
                curr = curr.add(1); // Skip the opening quote
                while *curr != b'"' && *curr != b'\0' {
                    curr = curr.add(1);
                }
                if *curr == b'"' {
                    *curr = b'\0'; // Null terminate for strcpy
                    curr = curr.add(1);
                }
            } else {
                while *curr != b' ' && *curr != b'\0' {
                    curr = curr.add(1);
                }
                if *curr != b'\0' {
                    *curr = b'\0'; // Null terminate for strcpy
                    curr = curr.add(1);
                }
            }

            let token_str = CString::from_raw(token_start);
            strs.push(token_str.into_raw());
            idx += 1;
        }

        strs.push(ptr::null_mut()); // Terminate the array
    }
    idx
}

fn execute(n_args: usize, args: Vec<*mut c_char>) {
    expect(args[0], "Command argument cannot be null");

    let do_wait = unsafe { *args[n_args - 2] != b'&' as i8 };
    let args = if do_wait {
        args
    } else {
        let mut args = args.clone();
        args[n_args - 2] = ptr::null_mut();
        args
    };

    let pid = unsafe { libc::fork() };

    match pid {
        -1 => expect(null_mut(), "Error running child process"),
        0 => {
            let _ = unsafe { libc::execvp(args[0], args.as_ptr() as *mut _) };
            expect(null_mut(), "Error");
        },
        _ => {
            if do_wait {
                let mut exit_status = 0;
                unsafe { libc::wait(&mut exit_status) };
            }
        }
    }

    for arg in args {
        if !arg.is_null() {
            unsafe { libc::free(arg as *mut c_void) };
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
                if let Some(ref cmd) = last_command {
                    println!("Exiting... Command: {}", cmd);
                }
                exit(0);
            }

            let n_args = get_n_spaces(&cmd) + 1;
            let mut cmd_args: Vec<*mut c_char> = Vec::new();
            tokenize(&cmd, &mut cmd_args);
            execute(n_args, cmd_args);
        }
    }
}

fn getc() -> Option<char> {
    let mut buffer = [0; 1];
    if std::io::stdin().read(&mut buffer).unwrap() == 0 {
        return None;
    }
    Some(buffer[0] as char)
}