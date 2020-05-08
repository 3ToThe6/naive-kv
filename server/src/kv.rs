use std::{
    collections::BTreeMap,
    fmt::{self, Display},
    fs, io,
    path::Path,
};

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    BinCode(bincode::Error),
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
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
            Self::Io(ref e) => e.fmt(f),
            Self::BinCode(ref e) => e.fmt(f),
        }
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;

/// An RAII key-value map manager. When dropped, it saves map to the file from which the map was tried to load.
pub struct KvManager<P: AsRef<Path>> {
    /// Key-value map. We use ``u64`` as key type instead of ``[u8; 8]``, because ``u64`` has a faster ``impl Ord``. We use ``Box<[u8]>`` as value type instead of ``[u8; 256]``, because ``[u8; 256]`` does not implement ``Serialize``.
    pub kv: BTreeMap<u64, Box<[u8]>>,
    path: P,
}

impl<P: AsRef<Path>> KvManager<P> {
    pub fn save(&self) -> Result<()> {
        fs::write(self.path.as_ref(), bincode::serialize(&self.kv)?)?;
        Ok(())
    }
}

impl<P: AsRef<Path>> Drop for KvManager<P> {
    fn drop(&mut self) {
        log::info!("Saving database to file...");
        if let Err(e) = self.save() {
            log::error!("Cannot save database: {}", e);
        }
    }
}
