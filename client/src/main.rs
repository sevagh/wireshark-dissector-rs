#![crate_type = "bin"]

extern crate nix;
extern crate libc;
extern crate core;
#[macro_use]
extern crate error_chain;
extern crate atbash;
extern crate common;

use nix::errno::Errno;
use nix::sys::socket;
use std::os::unix::io::RawFd;
use std::net::ToSocketAddrs;
use libc::{c_void, send};
use atbash::encode;
use common::{MSGLEN, SOCK};
use common::errors::*;

quick_main!(|| -> Result<i32> {
    let fd: Result<RawFd> = {
        let fd = socket::socket(
            socket::AddressFamily::Inet,
            socket::SockType::Stream,
            socket::SockFlag::empty(),
            socket::SOL_TCP,
        )?;
        let address = SOCK.to_socket_addrs()?.next().expect(
            "Cannot convert bind to socket address",
        );
        socket::connect(
            fd,
            &socket::SockAddr::new_inet(socket::InetAddr::from_std(&address)),
        )?;
        Ok(fd)
    };

    let fd = match fd {
        Ok(fd) => fd,
        Err(err) => {
            bail!("Socket error: {}", err);
        }
    };

    let version = encode("testver3").into_bytes();
    println!("VERSION LEN: {:#?}", version.len());
    let body = encode("hifriend").into_bytes();
    println!("BODY LEN: {:#?}", body.len());

    let mut payload = [&version[..], &body[..]].concat();
    println!("PAYLOAD LEN: {:#?}", payload.len());

    let nsent = unsafe { send(fd, payload.as_mut_ptr() as *mut c_void, MSGLEN, 0) };

    if nsent < 0 {
        bail!("recv failed: {}", Errno::last());
    } else {
        println!("Sent: {} bytes...\n", nsent);
    }

    Ok(0)
});
