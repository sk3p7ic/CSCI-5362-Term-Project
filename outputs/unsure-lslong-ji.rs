use std::ffi::{CString, CStr};
use std::fs::{self, File};
use std::io::{self, Write};
use std::libc::{self, c_void, c_char, stat, S_IFMT, S_IFREG, S_IFDIR, S_IFCHR, S_IFBLK};
use std::os::unix::fs::MetadataExt;
use std::ptr;
use std::time::{SystemTime,UNIX_EPOCH};

/// Ensures that a given pointer `ptr` is not null. If it is, it prints `msg` and exits.
macro_rules! expect {
    ($ptr:expr, $msg:expr) => {
        if $ptr.is_null() {
            eprintln!("{}", $msg);
            std::process::exit(1);
        }
    };
}

/// Converts a given UID `uid` to a Rust string of its name, if one exists.
fn uid_str(uid: u32) -> &'static str {
    unsafe {
        let pw = libc::getpwuid(uid);
        if pw.is_null() {
            return "Unknown";
        }
        CStr::from_ptr((*pw).pw_name).to_string_lossy().into_owned().as_str()
    }
}

/// Converts a given GID `gid` to a Rust string of its name, if one exists.
fn gid_str(gid: u32) -> &'static str {
    unsafe {
        let grp = libc::getgrgid(gid);
        if grp.is_null() {
            return "Unknown";
        }
        CStr::from_ptr((*grp).gr_name).to_string_lossy().into_owned().as_str()
    }
}

/// Writes the appropriate "rwx" values to a given buffer `str` having at least 3 characters.
fn permbits_to_chars(permission_value: u32, str: &mut [u8; 4]) {
    if str.len() < 4 {
        eprintln!("Not enough characters in string (Expected >= 3, Got {})", str.len());
        return;
    }
    str[0] = if (permission_value & 4) != 0 { b'r' } else { b'-' };
    str[1] = if (permission_value & 2) != 0 { b'w' } else { b'-' };
    str[2] = if (permission_value & 1) != 0 { b'x' } else { b'-' };
}

/// For some `mode`, converts it to a Rust string representation of the file type and permissions.
fn get_file_mode(mode: u32) -> String {
    let mut bits = vec![b'-'; 11];
    
    // Get the type
    bits[0] = match mode & S_IFMT {
        S_IFREG => b'-',
        S_IFDIR => b'd',
        S_IFCHR => b'c',
        S_IFBLK => b'b',
        _ => b'?',
    };
    
    // Get the permission values for owner, group, and others
    permbits_to_chars((mode >> 6) as u32, &mut bits[1..]);
    permbits_to_chars((mode >> 3) as u32, &mut bits[4..]);
    permbits_to_chars(mode as u32, &mut bits[7..]);
    
    String::from_utf8(bits).expect("Invalid UTF-8")
}

/// Displays information about a file `fname` with file information `info`.
fn display_file_info(fname: &str, info: &libc::stat) {
    print!("{}", get_file_mode(info.st_mode));
    print!("{:4} ", info.st_nlink);
    print!("{:<8} ", uid_str(info.st_uid));
    print!("{:<8} ", gid_str(info.st_gid));
    print!("{:8} ", info.st_size);
    let modified_time = SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(info.st_mtime as u64);
    if let Ok(datetime) = modified_time.duration_since(UNIX_EPOCH) {
        let formatted_time = datetime.as_secs();
        print!("{:.12} ", formatted_time);
    }
    println!("{}", fname);
}

/// A struct to represent the result of the `stat_file` function.
struct StatFileResult {
    info: libc::stat,
    ok: bool,
}

/// Retrieves information about a file if such a file exists.
fn stat_file(dirname: &str, fname: &str) -> StatFileResult {
    let mut res = StatFileResult {
        info: unsafe { std::mem::zeroed() },
        ok: false,
    };
    
    let full_path = format!("{}/{}", dirname, fname);
    if unsafe { libc::stat(full_path.as_ptr() as *const c_char, &mut res.info) } == -1 {
        // If the file still cannot be stat'd
        if unsafe { libc::stat(full_path.as_ptr() as *const c_char, &mut res.info) } == -1 {
            eprintln!("{}: {}", fname, io::Error::last_os_error());
            res.ok = false;
            return res;
        }
    }
    res.ok = true;
    return res;
}

/// Displays a listing of files contained in `dirname`.
fn display_dir(dirname: &str) {
    let entries = fs::read_dir(dirname).unwrap_or_else(|_| {
        eprintln!("Cannot open directory '{}'", dirname);
        std::process::exit(1);
    });
    
    for entry in entries.filter_map(Result::ok) {
        let fname = entry.file_name().into_string().unwrap(); // Potentially unsafe
        let stat_res = stat_file(dirname, &fname);
        if stat_res.ok {
            display_file_info(&fname, &stat_res.info);
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() == 1 {
        display_dir(".");
        return;
    }

    for dir in &args[1..] {
        println!("{}:", dir);
        display_dir(dir);
        // If there's still more directories to list, add a blank line
        if dir != args.last().unwrap() {
            println!();
        }
    }
}