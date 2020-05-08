use std::sync::{mpsc, Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

use actix_web::{error::ErrorInternalServerError as ise, web, Result};

use internal::*;
use kv::KvManager;

mod internal;
mod kv;

/// A combination of ``mpsc::Sender`` and ``KvManager``. When receiving a stop request, it will send a signal to the corresponding ``mpsc::Receiver`` returned by ``AppData::new_with_rx()``.
#[derive(Clone)]
pub struct AppData {
    tx: mpsc::Sender<()>,
    kvmg: Arc<RwLock<KvManager<&'static str>>>,
}

impl AppData {
    pub fn new_with_rx() -> kv::Result<(Self, mpsc::Receiver<()>)> {
        let (tx, rx) = mpsc::channel();
        let kvmg = Arc::new(RwLock::new(KvManager::new("kv.dat")?));
        Ok((Self { tx, kvmg }, rx))
    }
    fn kvmg(&self) -> Result<RwLockReadGuard<KvManager<&'static str>>> {
        self.kvmg.read().map_err(|e| ise(e.to_string()))
    }
    fn kvmg_mut(&self) -> Result<RwLockWriteGuard<KvManager<&'static str>>> {
        self.kvmg.write().map_err(|e| ise(e.to_string()))
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get)
        .service(put)
        .service(put)
        .service(delete)
        .service(stop);
}
