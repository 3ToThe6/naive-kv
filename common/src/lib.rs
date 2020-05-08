use std::net::ToSocketAddrs;

pub mod req;

pub fn server_address() -> impl AsRef<str> {
    "127.0.0.1:80"
}
