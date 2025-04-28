use std::collections::HashMap;
use std::ffi::CString;
use std::fs::{self, DirEntry};
use std::os::unix::fs::MetadataExt;
use std::path::Path;
use std::ptr;
use std::time::{SystemTime, UNIX_EPOCH};

fn expect<T>(ptr: Option<T>, msg: &str) {
    if ptr.is_none() {
        eprintln!("{}", msg);
        std::process::exit(1);
    }
}

fn uid_str(uid: u32) -> String {
    match users::get_user_by_uid(uid) {
        Some(user) => user.name().to_string(),
        None => "Unknown".to_string(),
    }
}

fn gid_str(gid: u32) -> String {
    match users::get_group_by_gid(gid) {
        Some(group) => group.name().to_string(),
        None => "Unknown".to_string(),
    }
}

fn permbits_to_chars(permission_value: u32) -> String {
    let mut chars = String::from("----------");
    if permission_value & 0b100 != 0 {
        chars.replace_range(0..1, "r");
    }
    if permission_value & 0b010 != 0 {
        chars.replace_range(1..2, "w");
    }
    if permission_value & 0b001 != 0 {
        chars.replace_range(2..3, "x");
    }
    chars
}

fn get_file_mode(mode: u32) -> String {
    let mut bits = String::from("----------");

    bits.replace_range(0..1, match mode & libc::S_IFMT {
        libc::S_IFREG => "-",
        libc::S_IFDIR => "d",
        libc::S_IFCHR => "c",
        libc::S_IFBLK => "b",
        _ => "?",
    });

    bits.replace_range(1..4, &permbits_to_chars(mode >> 6));
    bits.replace_range(4..7, &permbits_to_chars(mode >> 3));
    bits.replace_range(7..10, &permbits_to_chars(mode));

    bits
}

fn display_file_info(fname: &str, info: &fs::Metadata) {
    println!(
        "{} {:>4} {:<8} {:<8} {:>8} {:.12} {}",
        get_file_mode(info.mode()),
        info.nlink(),
        uid_str(info.uid()),
        gid_str(info.gid()),
        info.len(),
        time_to_string(info.modified().unwrap_or(SystemTime::now())),
        fname
    );
}

fn time_to_string(time: SystemTime) -> String {
    let duration = time.duration_since(UNIX_EPOCH).expect("Time went backwards");
    let datetime = chrono::NaiveDateTime::from_timestamp(duration.as_secs() as i64, 0);
    datetime.format("%b %d %H:%M").to_string()
}

#[derive(Debug)]
struct StatFileResult {
    info: fs::Metadata,
    ok: bool,
}

fn stat_file(dirname: &str, fname: &str) -> StatFileResult {
    let full_path = Path::new(dirname).join(fname);
    match full_path.metadata() {
        Ok(info) => StatFileResult { info, ok: true },
        Err(_) => StatFileResult { info: fs::Metadata::default(), ok: false },
    }
}

fn display_dir(dirname: &str) {
    match fs::read_dir(dirname) {
        Ok(entries) => {
            for entry in entries.filter_map(Result::ok) {
                let stat_res = stat_file(dirname, entry.file_name().to_str().unwrap());
                if stat_res.ok {
                    display_file_info(entry.file_name().to_str().unwrap(), &stat_res.info);
                }
            }
        },
        Err(e) => {
            eprintln!("Cannot open directory '{}': {}", dirname, e);
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 1 {
        display_dir(".");
    } else {
        for arg in args.iter().skip(1) {
            println!("{}:", arg);
            display_dir(arg);
            if arg != args.last().unwrap() {
                println!();
            }
        }
    }
}