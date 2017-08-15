#[macro_use]
extern crate error_chain;
extern crate nix;

pub static SOCK: &'static str = "127.0.0.1:8888";

/* version = 8 bytes, body = 8 bytes */
pub static MSGLEN: usize = 16;

pub mod errors {
    error_chain! {
        foreign_links {
            Nix(::nix::Error);
            Io(::std::io::Error);
        }
    }
}
