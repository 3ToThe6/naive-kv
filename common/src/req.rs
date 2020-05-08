use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Get {
    pub key: u64,
}

#[derive(Deserialize, Serialize)]
pub struct Scan {
    /// Start key. ``None`` means unbounded, and the boolean is whether this start key is exclusive.
    pub start: Option<(u64, bool)>,
    /// End key. ``None`` means unbounded, and the boolean is whether this end key is inclusive.
    pub end: Option<(u64, bool)>,
}

#[derive(Deserialize, Serialize)]
pub struct Put {
    pub key: u64,
    pub value: Box<[u8]>,
}

#[derive(Deserialize, Serialize)]
pub struct Delete {
    pub key: u64,
}
