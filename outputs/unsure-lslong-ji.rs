use std::ffi::{CString, CStr};
use std::fs::{self, File};
use std::io::{self, Write};
use std::os::unix::fs::MetadataExt;
use std::os::unix::ffi::OsStrExt;
use std::path::Path;
use std::ptr;
use std::time::{UNIX_EPOCH, SystemTime};
use std::process::exit;
use std::string::ToString;

#[inline]
fn uid_str(uid: u32) -> &'static str {
    match users::get_user_by_uid(uid) {
        Some(user) => user.name().to_str().unwrap_or("Unknown"),
        None => "Unknown",
    }
}

#[inline]
fn gid_str(gid: u32) -> &'static str {
    match users::get_group_by_gid(gid) {
        Some(group) => group.name().to_str().unwrap_or("Unknown"),
        None => "Unknown",
    }
}

#[inline]
fn permbits_to_chars(permission_value: u32) -> String {
    let mut chars = String::from("---");
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

fn get_filemode(mode: u32) -> String {
    let mut bits = String::from("----------");

    // Get the type
    match mode & 0o170000 {
        0o100000 => bits.replace_range(0..1, "-"), // Regular file
        0o40000 => bits.replace_range(0..1, "d"),  // Directory
        0o20000 => bits.replace_range(0..1, "c"),  // Char I/O device
        0o60000 => bits.replace_range(0..1, "b"),  // Blk I/O device
        _ => bits.replace_range(0..1, "?"),         // Other types
    }

    // Get permission values for owner, group, and others
    bits.replace_range(1..4, &permbits_to_chars((mode >> 6) as u32));
    bits.replace_range(4..7, &permbits_to_chars((mode >> 3) as u32));
    bits.replace_range(7..10, &permbits_to_chars(mode as u32));

    bits
}

fn display_file_info(fname: &str, metadata: &fs::Metadata) {
    print!("{}", get_filemode(metadata.mode()));
    print!("{:4} ", metadata.nlink());
    print!("{:<8} ", uid_str(metadata.uid()));
    print!("{:<8} ", gid_str(metadata.gid()));
    print!("{:8} ", metadata.len());
    print!("{:.12} ", format!("{:?}", SystemTime::now() - metadata.modified().unwrap_or(UNIX_EPOCH)));
    println!("{}", fname);
}

struct StatFileResult {
    info: fs::Metadata,
    ok: bool,
}

fn stat_file(dirname: &Path, fname: &str) -> StatFileResult {
    let full_path = dirname.join(fname);
    match full_path.metadata() {
        Ok(info) => StatFileResult { info, ok: true },
        Err(_) => {
            StatFileResult { info: fs::File::open(full_path).unwrap().metadata().unwrap(), ok: false }
        }
    }
}

fn display_dir(dirname: &str) {
    let dir = fs::read_dir(dirname);
    match dir {
        Ok(entries) => {
            for entry in entries.filter_map(Result::ok) {
                let stat_res = stat_file(Path::new(dirname), entry.file_name().to_str().unwrap());
                if stat_res.ok {
                    display_file_info(entry.file_name().to_str().unwrap(), &stat_res.info);
                }
            }
        }
        Err(e) => {
            eprintln!("Cannot open directory '{}': {}", dirname, e);
        }
    }
}

fn main() {
    let args: Vec<_> = std::env::args().collect();

    if args.len() == 1 {
        display_dir(".");
        exit(0);
    }

    for arg in &args[1..] {
        println!("{}:", arg);
        display_dir(arg);
        if arg != args.last().unwrap() {
            println!();
        }
    }
}