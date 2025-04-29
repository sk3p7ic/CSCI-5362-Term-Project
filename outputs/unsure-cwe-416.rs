use std::ffi::CStr;
use std::ptr;
use std::os::raw::c_char;
use std::alloc::{alloc, dealloc, Layout};

const BUFSIZER1: usize = 512;
const BUFSIZER2: usize = (BUFSIZER1 / 2) - 8;

fn main(argc: isize, argv: *mut *mut c_char) {
    unsafe {
        let buf1_r1 = alloc(Layout::from_size_align(BUFSIZER1, 1).unwrap());
        let buf2_r1 = alloc(Layout::from_size_align(BUFSIZER1, 1).unwrap());
        
        dealloc(buf2_r1, Layout::from_size_align(BUFSIZER1, 1).unwrap());

        let buf2_r2 = alloc(Layout::from_size_align(BUFSIZER2, 1).unwrap());
        let buf3_r2 = alloc(Layout::from_size_align(BUFSIZER2, 1).unwrap());

        let arg1 = CStr::from_ptr(*argv.offset(1)).to_str().unwrap();
        let bytes_to_copy = BUFSIZER1 - 1;
        let slice = &mut *(buf2_r1 as *mut [u8; BUFSIZER1]);
        slice[..bytes_to_copy].copy_from_slice(&arg1.as_bytes()[..bytes_to_copy]);

        dealloc(buf1_r1, Layout::from_size_align(BUFSIZER1, 1).unwrap());
        dealloc(buf2_r2, Layout::from_size_align(BUFSIZER2, 1).unwrap());
        dealloc(buf3_r2, Layout::from_size_align(BUFSIZER2, 1).unwrap());
    }
}