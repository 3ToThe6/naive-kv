use std::fmt::{self, Display};

use reqwest::blocking::Client;

use common::*;
pub use opt::Opt;

mod opt;

#[derive(Debug)]
pub enum Error {
    Reqwest(reqwest::Error),
    BinCode(bincode::Error),
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Self::Reqwest(error)
    }
}

impl From<bincode::Error> for Error {
    fn from(error: bincode::Error) -> Self {
        Self::BinCode(error)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Reqwest(ref e) => e.fmt(f),
            Self::BinCode(ref e) => e.fmt(f),
        }
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;

pub fn get(key: u64) -> Result<Option<Box<[u8]>>> {
    let request = Client::new().get(server_url()).json(&req::Get { key });
    Ok(bincode::deserialize(&request.send()?.bytes()?)?)
}

pub fn scan(start: Option<(u64, bool)>, end: Option<(u64, bool)>) -> Result<Vec<(u64, Box<u8>)>> {
    let mut url = server_url();
    url.set_path("/scan");
    let request = Client::new().get(url).json(&req::Scan { start, end });
    Ok(bincode::deserialize(&request.send()?.bytes()?)?)
}

pub fn put(key: u64, value: &[u8]) -> reqwest::Result<()> {
    if value.len() != 256 {
        panic!("value.len() != 256");
    }
    Client::new()
        .put(server_url())
        .json(&req::Put { key, value })
        .send()?;
    Ok(())
}

pub fn delete(key: u64) -> reqwest::Result<()> {
    Client::new()
        .delete(server_url())
        .json(&req::Delete { key })
        .send()?;
    Ok(())
}
