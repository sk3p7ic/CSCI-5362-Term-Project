use std::ffi::CString;
use std::ptr;
use std::net::{IpAddr, Ipv4Addr};
use std::os::raw::c_char;

extern "C" {
    fn gethostbyaddr(
        addr: *const std::os::raw::c_void,
        len: i32,
        type_: i32,
    ) -> *mut hostent;
}

#[repr(C)]
struct hostent {
    h_name: *mut c_char,
    h_aliases: *mut *mut c_char,
    h_addrtype: i32,
    h_length: i32,
    h_addr_list: *mut *mut c_char,
}

fn validate_addr_form(user_supplied_addr: &str) {
    // Dummy validation for address form. In a real scenario,
    // you would implement proper address validation.
}

fn host_lookup(user_supplied_addr: &str) {
    let addr: Ipv4Addr = user_supplied_addr.parse().expect("Invalid IP address format");
    let addr_ptr = &addr as *const Ipv4Addr as *const std::os::raw::c_void;
    let mut hostname = vec![0; 64];

    validate_addr_form(user_supplied_addr);

    unsafe {
        let hp = gethostbyaddr(addr_ptr, std::mem::size_of::<Ipv4Addr>() as i32, 2);
        if hp.is_null() {
            panic!("gethostbyaddr failed");
        }

        let name = (*hp).h_name;
        std::ptr::copy_nonoverlapping(name, hostname.as_mut_ptr() as *mut c_char, hostname.len());
    }

    println!("Hostname: {:?}", CString::from_vec_lossy(hostname));
}