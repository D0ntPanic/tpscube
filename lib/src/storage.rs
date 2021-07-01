use anyhow::Result;
use std::collections::BTreeMap;

#[cfg(feature = "native-storage")]
use rocksdb::{DBCompressionType, Options, DB};
#[cfg(feature = "native-storage")]
use std::path::Path;

#[cfg(feature = "web-storage")]
use anyhow::anyhow;

pub(crate) trait Storage {
    fn get(&self, key: &str) -> Result<Option<Vec<u8>>>;
    fn put(&mut self, key: &str, value: &[u8]) -> Result<()>;
    fn delete(&mut self, key: &str) -> Result<()>;
    fn flush(&self);
}

pub(crate) struct TemporaryStorage {
    storage: BTreeMap<String, Vec<u8>>,
}

#[cfg(feature = "native-storage")]
pub(crate) struct RocksDBStorage {
    db: DB,
}

#[cfg(feature = "web-storage")]
pub(crate) struct WebStorage;

impl TemporaryStorage {
    pub fn new() -> Self {
        Self {
            storage: BTreeMap::new(),
        }
    }
}

impl Storage for TemporaryStorage {
    fn get(&self, key: &str) -> Result<Option<Vec<u8>>> {
        Ok(self.storage.get(key).cloned())
    }

    fn put(&mut self, key: &str, value: &[u8]) -> Result<()> {
        self.storage.insert(key.into(), value.to_vec());
        Ok(())
    }

    fn delete(&mut self, key: &str) -> Result<()> {
        self.storage.remove(key);
        Ok(())
    }

    fn flush(&self) {}
}

#[cfg(feature = "native-storage")]
impl RocksDBStorage {
    pub fn open(path: &Path) -> Result<Self> {
        let mut opts = Options::default();
        opts.create_if_missing(true);
        opts.set_compression_type(DBCompressionType::Zstd);
        opts.set_keep_log_file_num(8);
        let db = DB::open(&opts, path)?;
        Ok(Self { db })
    }
}

#[cfg(feature = "native-storage")]
impl Storage for RocksDBStorage {
    fn get(&self, key: &str) -> Result<Option<Vec<u8>>> {
        Ok(self.db.get(key)?)
    }

    fn put(&mut self, key: &str, value: &[u8]) -> Result<()> {
        Ok(self.db.put(key, value)?)
    }

    fn delete(&mut self, key: &str) -> Result<()> {
        Ok(self.db.delete(key)?)
    }

    fn flush(&self) {
        let _ = self.db.flush();
    }
}

#[cfg(feature = "web-storage")]
fn local_storage() -> Result<web_sys::Storage> {
    web_sys::window()
        .ok_or_else(|| anyhow!("No window"))?
        .local_storage()
        .map_err(|_| anyhow!("Failed to access local storage"))?
        .ok_or_else(|| anyhow!("No local storage"))
}

#[cfg(feature = "web-storage")]
impl Storage for WebStorage {
    fn get(&self, key: &str) -> Result<Option<Vec<u8>>> {
        if let Some(string) = local_storage()?
            .get_item(key)
            .map_err(|_| anyhow!("Failed to fetch item from local storage"))?
        {
            Ok(base64::decode(string)
                .map(|bytes| Some(bytes))
                .unwrap_or(None))
        } else {
            Ok(None)
        }
    }

    fn put(&mut self, key: &str, value: &[u8]) -> Result<()> {
        local_storage()?
            .set_item(key, &base64::encode(value))
            .map_err(|_| anyhow!("Failed to store item in local storage"))
    }

    fn delete(&mut self, key: &str) -> Result<()> {
        local_storage()?
            .remove_item(key)
            .map_err(|_| anyhow!("Failed to delete item from local storage"))
    }

    fn flush(&self) {}
}
