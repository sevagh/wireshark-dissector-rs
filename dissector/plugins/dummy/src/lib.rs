#![crate_type = "staticlib"]

#[no_mangle]
pub extern fn dissect_dummy_rs(data: &[u8]) -> i32 {
    println!("This is being printed from Rust!!!");
    0
}
