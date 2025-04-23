use std::ptr;
use std::ffi::CString;
use std::os::raw::c_char;

fn main() {
    let msg = CString::new("Hello, world!").unwrap();
    let len = msg.to_bytes().len();
    
    // Allocating memory using `std::alloc` to simulate the usage of malloc
    let buff: *mut c_char = unsafe { std::alloc::alloc(std::alloc::Layout::from_size_align(len, 1).unwrap()) };
    
    unsafe {
        // Simulating the buffer overflow by not null-terminating
        ptr::copy_nonoverlapping(msg.as_ptr(), buff, len);
        // Intentionally accessing uninitialized area
        *buff.add(len) = 0; // Null-terminate for printing

        // Printing the string from the allocated buffer
        println!("{}", std::ffi::CStr::from_ptr(buff).to_string_lossy());

        // Freeing the memory allocated with malloc
        std::alloc::dealloc(buff, std::alloc::Layout::from_size_align(len, 1).unwrap());
    }
}