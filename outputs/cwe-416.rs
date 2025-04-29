use std::alloc::{alloc, dealloc, Layout};
use std::ffi::CStr;
use std::os::raw::c_char;
use std::ptr;

const BUFSIZER1: usize = 512;
const BUFSIZER2: usize = (BUFSIZER1 / 2) - 8;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <input>", args[0]);
        return;
    }

    unsafe {
        let buf1R1_layout = Layout::from_size_align(BUFSIZER1, 1).unwrap();
        let buf2R1_layout = Layout::from_size_align(BUFSIZER1, 1).unwrap();
        let buf2R2_layout = Layout::from_size_align(BUFSIZER2, 1).unwrap();
        let buf3R2_layout = Layout::from_size_align(BUFSIZER2, 1).unwrap();

        let buf1R1 = alloc(buf1R1_layout);
        let buf2R1 = alloc(buf2R1_layout);
        dealloc(buf2R1, buf2R1_layout); // freeing buf2R1 immediately

        let buf2R2 = alloc(buf2R2_layout);
        let buf3R2 = alloc(buf3R2_layout);
        
        // Copy the string argument into buf2R1
        let input = CStr::from_bytes_with_nul(args[1].as_bytes()).unwrap();
        ptr::copy_nonoverlapping(input.as_ptr(), buf2R1 as *mut c_char, BUFSIZER1 - 1);

        dealloc(buf1R1, buf1R1_layout);
        dealloc(buf2R2, buf2R2_layout);
        dealloc(buf3R2, buf3R2_layout);
    }
}