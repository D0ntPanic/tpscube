use crate::future::spawn_future;
use anyhow::Result;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

#[cfg(feature = "native-storage")]
use rocksdb::{DBCompressionType, Options, DB};
#[cfg(feature = "native-storage")]
use std::path::Path;

#[cfg(feature = "web-storage")]
use anyhow::anyhow;

#[cfg(feature = "native-storage")]
pub(crate) struct Storage {
    db: DB,
}

#[cfg(feature = "web-storage")]
pub(crate) struct Storage;

pub(crate) struct DeferredStorage {
    error: Arc<Mutex<Option<String>>>,
    queue: Arc<Mutex<StorageQueue>>,
}

enum StorageQueueItem {
    Put(String, Vec<u8>),
    Delete(String),
    Flush,
}

struct StorageQueue {
    storage: Option<Storage>,
    items: VecDeque<StorageQueueItem>,
}

#[cfg(feature = "native-storage")]
impl Storage {
    pub fn open(path: &Path) -> Result<Self> {
        let mut opts = Options::default();
        opts.create_if_missing(true);
        opts.set_compression_type(DBCompressionType::Zstd);
        opts.set_keep_log_file_num(8);
        let db = DB::open(&opts, path)?;
        Ok(Self { db })
    }

    pub async fn get(&self, key: &str) -> Result<Option<Vec<u8>>> {
        Ok(self.db.get(key)?)
    }

    pub async fn put(&mut self, key: &str, value: &[u8]) -> Result<()> {
        Ok(self.db.put(key, value)?)
    }

    pub async fn delete(&mut self, key: &str) -> Result<()> {
        Ok(self.db.delete(key)?)
    }

    pub async fn flush(&self) {
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
impl Storage {
    pub async fn get(&self, key: &str) -> Result<Option<Vec<u8>>> {
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

    pub async fn put(&mut self, key: &str, value: &[u8]) -> Result<()> {
        local_storage()?
            .set_item(key, &base64::encode(value))
            .map_err(|_| anyhow!("Failed to store item in local storage"))
    }

    pub async fn delete(&mut self, key: &str) -> Result<()> {
        local_storage()?
            .remove_item(key)
            .map_err(|_| anyhow!("Failed to delete item from local storage"))
    }

    pub async fn flush(&self) {}
}

impl DeferredStorage {
    pub fn new(storage: Storage) -> Self {
        Self {
            error: Arc::new(Mutex::new(None)),
            queue: Arc::new(Mutex::new(StorageQueue {
                storage: Some(storage),
                items: VecDeque::new(),
            })),
        }
    }

    fn push(&self, item: StorageQueueItem) {
        let mut queue = self.queue.lock().unwrap();
        queue.items.push_back(item);

        // If this object owns the storage, we don't have a future running to handle
        // the queue. Start one if there isn't one running.
        if queue.storage.is_some() {
            // Hand off ownership of the storage to the future. This will ensure there is
            // exactly one owner of the storage and prevent the need to lock while accessing
            // storage, which would not be easily doable in async context, as holding
            // the lock while waiting on the storage implementation could yield deadlocks.
            // This will also clear the storage from the `DeferredStorage` instance, which
            // acts as a flag that a future owns the storage and is ready to accept queue
            // items for processing.
            let mut storage = queue.storage.take().unwrap();

            // Spawn a future to handle storage requests
            let queue = self.queue.clone();
            let error_message = self.error.clone();
            spawn_future(async move {
                loop {
                    let item = {
                        // Lock the queue lock only while checking for new items
                        let mut queue = queue.lock().unwrap();
                        let result = queue.items.pop_front();
                        match result {
                            Some(item) => item,
                            None => {
                                // There are no more entries in the queue. Hand ownership of
                                // the storage implementation back to the `DeferredStorage`
                                // instance. If another request comes in, a new future will
                                // be started. This must be done while the lock is held to
                                // avoid race conditions on whether there is a future ready
                                // to handle queue items or not.
                                queue.storage = Some(storage);
                                break;
                            }
                        }
                    };

                    // Handle the queue item. The lock must not be held here, as we may
                    // need to `await` the storage implementation.
                    match item {
                        StorageQueueItem::Put(key, value) => {
                            match storage.put(&key, &value).await {
                                Ok(_) => (),
                                Err(error) => {
                                    // On error set the error string and abort all future work,
                                    // as it may corrupt the database. Simply not handing the
                                    // storage ownership back to the `DeferredStorage` instance
                                    // will cause there to be no more work completed.
                                    *error_message.lock().unwrap() = Some(error.to_string());
                                    break;
                                }
                            }
                        }
                        StorageQueueItem::Delete(key) => {
                            match storage.delete(&key).await {
                                Ok(_) => (),
                                Err(error) => {
                                    // On error set the error string and abort all future work,
                                    // as it may corrupt the database. Simply not handing the
                                    // storage ownership back to the `DeferredStorage` instance
                                    // will cause there to be no more work completed.
                                    *error_message.lock().unwrap() = Some(error.to_string());
                                    break;
                                }
                            }
                        }
                        StorageQueueItem::Flush => storage.flush().await,
                    }
                }
            });
        }
    }

    pub fn put(&self, key: &str, value: &[u8]) {
        self.push(StorageQueueItem::Put(key.to_string(), value.to_vec()));
    }

    pub fn delete(&self, key: &str) {
        self.push(StorageQueueItem::Delete(key.to_string()));
    }

    pub fn flush(&self) {
        self.push(StorageQueueItem::Flush);
    }

    pub fn check_for_error(&self) -> Option<String> {
        self.error.lock().unwrap().clone()
    }
}
