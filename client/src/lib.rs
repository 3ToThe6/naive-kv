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

pub fn scan(start: Option<(u64, bool)>, end: Option<(u64, bool)>) -> Result<Vec<(u64, Box<[u8]>)>> {
    if let (Some((s, se)), Some((e, ei))) = (start, end) {
        if s > e || (s == e && se && !ei) {
            panic!("Invalid range");
        }
    }
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

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn main() {
        // Put
        put(0, &[0; 256]).unwrap();
        put(1, &[1; 256]).unwrap();
        put(2, &[2; 256]).unwrap();
        put(3, &[3; 256]).unwrap();
        // Delete
        delete(2).unwrap();
        // Get
        assert_eq!(get(2).unwrap(), None);
        assert_eq!(get(4).unwrap(), None);
        assert_eq!(get(3).unwrap(), Some(vec![3; 256].into_boxed_slice()));
        // Scan
        assert_eq!(
            scan(Some((0, true)), None).unwrap(),
            vec![
                (1, vec![1; 256].into_boxed_slice()),
                (3, vec![3; 256].into_boxed_slice()),
            ],
        );
    }
}
