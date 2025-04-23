fn main() {
    let msg: [i8; 14] = *b"Hello, World!\0";
    unsafe {
        libc::printf(b"%s\n\0".as_ptr() as *const i8, msg.as_ptr());
    }
}