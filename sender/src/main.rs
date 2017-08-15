#![crate_type = "bin"]

extern crate nix;
extern crate libc;
extern crate core;
#[macro_use]
extern crate error_chain;
extern crate atbash;

use nix::errno::Errno;
use nix::sys::socket;
use std::os::unix::io::RawFd;
use std::net::ToSocketAddrs;
use libc::{c_void, recv, MSG_WAITALL};
use atbash::decode;

static SOCK: &'static str = "127.0.0.1:8888";

/* version = 8 bytes, body = 8 bytes */
static MSGLEN: usize = 16;

pub mod errors {
    error_chain! {
        foreign_links {
            Nix(::nix::Error);
            Io(::std::io::Error);
        }
    }
}

use errors::*;

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
        socket::bind(
            fd,
            &socket::SockAddr::new_inet(socket::InetAddr::from_std(&address)),
        )?;
        socket::listen(fd, 16).expect("Couldn't listen for connections");
        Ok(fd)
    };

    let fd = match fd {
        Ok(fd) => fd,
        Err(err) => {
            bail!("Socket error: {}", err);
        }
    };

    let mut payload = vec![0u8; MSGLEN];

    let mut fd_ = socket::accept(fd)?;
    loop {
        let nread = unsafe {
            recv(
                fd_,
                payload.as_mut_ptr() as *mut c_void,
                MSGLEN,
                MSG_WAITALL,
            )
        };
        if nread == 0 {
            fd_ = socket::accept(fd)?;
            continue;
        } else if nread < 1 {
            bail!("recvmmsg failed: {}", Errno::last());
        } else if nread != MSGLEN as isize {
            bail!("Incomplete read of size {}", nread)
        }
        println!("Read {} bytes...\n", nread);

        let vers = decode(&String::from_utf8_lossy(&payload[..8]));
        let body = decode(&String::from_utf8_lossy(&payload[8..16]));

        println!("\tversion: {:?}\n\tbody: {:?}", vers, body);
    }
});
