use std::ffi::CString;
use std::ptr;

fn main() {
    let msg = CString::new("Hello, world!").unwrap();
    let mut buff: *mut i8 = unsafe { libc::malloc(msg.to_bytes().len()) as *mut i8 };
    unsafe {
        ptr::copy_nonoverlapping(msg.as_ptr(), buff, msg.to_bytes().len());
        println!("{}", CString::from_raw(buff).to_string_lossy());
    }
}