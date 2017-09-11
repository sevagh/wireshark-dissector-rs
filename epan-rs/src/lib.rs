#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;
    
    #[test]
    fn test_proto_register_protocol() {
        let proto_dummy = unsafe {
            proto_register_protocol(
                CString::new("Dummy Protocol").unwrap().as_ptr(),
                CString::new("DUMMY").unwrap().as_ptr(),
                CString::new("dummy").unwrap().as_ptr());
        };
    }
}
