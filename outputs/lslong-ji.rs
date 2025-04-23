use std::ffi::CString;
use std::fs::{self, DirEntry, Metadata};
use std::os::unix::fs::MetadataExt;
use std::ptr;
use std::time::{SystemTime, UNIX_EPOCH};
use std::vec::Vec;

fn uid_to_string(uid: u32) -> String {
    let pw_entry = unsafe { libc::getpwuid(uid) };
    if pw_entry.is_null() {
        return "Unknown".to_string();
    }
    unsafe { CString::from_raw((*pw_entry).pw_name).into_string().unwrap() }
}

fn gid_to_string(gid: u32) -> String {
    let gr_entry = unsafe { libc::getgrgid(gid) };
    if gr_entry.is_null() {
        return "Unknown".to_string();
    }
    unsafe { CString::from_raw((*gr_entry).gr_name).into_string().unwrap() }
}

fn permbits_to_chars(permission_value: u32, str: &mut [char; 4]) {
    if permission_value & 4 != 0 {
        str[0] = 'r';
    }
    if permission_value & 2 != 0 {
        str[1] = 'w';
    }
    if permission_value & 1 != 0 {
        str[2] = 'x';
    }
}

fn get_file_mode(mode: u32) -> String {
    let mut bits = ['-'; 11];
    bits[0] = match mode & libc::S_IFMT {
        libc::S_IFREG => '-',
        libc::S_IFDIR => 'd',
        libc::S_IFCHR => 'c',
        libc::S_IFBLK => 'b',
        _ => '?',
    };

    let permissions = (mode >> 6) as u32;
    permbits_to_chars(permissions, &mut bits[1..4]);
    let permissions = (mode >> 3) as u32;
    permbits_to_chars(permissions, &mut bits[4..7]);
    let permissions = mode as u32;
    permbits_to_chars(permissions, &mut bits[7..10]);

    bits.iter().collect()
}

fn display_file_info(fname: &str, metadata: &Metadata) {
    let mode_str = get_file_mode(metadata.mode());
    let links = metadata.nlink();
    let uid_str = uid_to_string(metadata.uid());
    let gid_str = gid_to_string(metadata.gid());
    let size = metadata.len();
    
    let modified_time = metadata.modified().unwrap_or(SystemTime::now());
    let duration = modified_time.duration_since(UNIX_EPOCH).unwrap();
    let timestamp = duration.as_secs();

    println!("{} {} {:>4} {:<8} {:<8} {:>8} {:.12}", 
             mode_str, 
             links, 
             uid_str, 
             gid_str, 
             size, 
             timestamp);
}

fn display_dir(dirname: &str) {
    match fs::read_dir(dirname) {
        Ok(entries) => {
            for entry in entries.filter_map(Result::ok) {
                let fname = entry.file_name().into_string().unwrap();
                let metadata = entry.metadata().unwrap();
                display_file_info(&fname, &metadata);
            }
        }
        Err(err) => {
            eprintln!("Cannot open directory '{}': {}", dirname, err);
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 1 {
        display_dir(".");
        return;
    }

    for arg in &args[1..] {
        println!("{}:", arg);
        display_dir(arg);
        if arg != args.last().unwrap() {
            println!();
        }
    }
}