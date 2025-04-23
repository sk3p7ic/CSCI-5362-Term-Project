use std::ptr;

fn main() {
    let msg = "Hello, world!";
    
    // Incorrect buffer allocation: insufficient size
    let mut buff: *mut i8 = unsafe { libc::malloc(10) as *mut i8 };
    
    // Check if buff is NULL (unsafe dereference)
    if buff.is_null() {
        unsafe { libc::free(buff as *mut libc::c_void) }; // Still trying to free even if it was null
        return;
    }

    // This copies the string but doesn't account for the null terminator
    unsafe {
        ptr::copy_nonoverlapping(msg.as_ptr(), buff, 10); // Unsafe, copying over unallocated space
        println!("{}", std::ffi::CStr::from_ptr(buff).to_string_lossy());
    }
    
    // Memory leak: 'buff' is not freed
}