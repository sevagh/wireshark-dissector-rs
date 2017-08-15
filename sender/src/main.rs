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
static MSGLEN: usize = 32 * 8;

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

    let mut payload = vec![0u8; 32];

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
            panic!("recvmmsg failed: {}", Errno::last());
        }
        println!("Read {} bytes...\n", nread);

        let version = decode(&String::from_utf8_lossy(&payload[..7]));
        let verb = decode(&String::from_utf8_lossy(&payload[8..15]));
        let body = decode(&String::from_utf8_lossy(&payload[16..]));

        println!(
            "\tversion: {:?}\n\tverb: {:?}\n\tbody: {:?}",
            version,
            verb,
            body
        );
    }
});
