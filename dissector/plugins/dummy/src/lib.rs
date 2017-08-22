#![crate_type = "staticlib"]

extern crate common;
extern crate transform;
extern crate libc;

use common::MSGLEN;
use transform::decode;
use libc::{memcpy, c_void};

#[no_mangle]
pub extern "C" fn dissect_dummy_rs(data: &mut [u8]) -> i32 {
    if data.len() > 2*MSGLEN {
        return -1;
    }

    println!("Data length: {:#?}", data.len());

    let mut decoded_version = decode(&data[..8]);
    let mut decoded_body = decode(&data[8..]);

    unsafe {
        memcpy(
            data.as_mut_ptr() as *mut c_void,
            decoded_version.as_mut_ptr() as *mut c_void,
            MSGLEN * 8,
        );
        memcpy(
            data[8..].as_mut_ptr() as *mut c_void,
            decoded_body.as_mut_ptr() as *mut c_void,
            MSGLEN * 8,
        );
    }

    println!("Decoded ver: {}", String::from_utf8_lossy(&decoded_version));
    println!("Decoded body: {}", String::from_utf8_lossy(&decoded_body));
    0
}
