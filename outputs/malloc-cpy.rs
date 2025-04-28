use std::ffi::CString;
use std::ptr;

fn main() {
    let msg = "Hello, world!";
    let buff = CString::new(msg).unwrap();
    let c_str = buff.as_ptr();
    
    unsafe {
        // Print the C-style string
        println!("{}", std::ffi::CStr::from_ptr(c_str).to_string_lossy());
    }
}