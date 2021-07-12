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
#[cfg(feature = "web-storage")]
use js_sys::{Function, Promise, Uint8Array};
#[cfg(feature = "web-storage")]
use wasm_bindgen::{closure::Closure, JsCast, JsValue};
#[cfg(feature = "web-storage")]
use wasm_bindgen_futures::JsFuture;
#[cfg(feature = "web-storage")]
use web_sys::{IdbDatabase, IdbTransactionMode};

#[cfg(feature = "native-storage")]
pub(crate) struct Storage {
    db: DB,
}

#[cfg(feature = "web-storage")]
pub(crate) struct Storage {
    db: IdbDatabase,
}

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
impl Storage {
    pub async fn new() -> Result<Self> {
        // Create database open request
        let open_request = web_sys::window()
            .ok_or_else(|| anyhow!("No window"))?
            .indexed_db()
            .map_err(|_| anyhow!("Failed to access IndexedDB"))?
            .ok_or_else(|| anyhow!("No IndexedDB found"))?
            .open("tpscube")
            .map_err(|_| anyhow!("Failed to create IndexedDB open request"))?;

        // Create a `Promise` object for the request so that we can obtain a future
        let open_request_copy = open_request.clone();
        let mut promise = move |resolve: Function, reject: Function| {
            let resolve_request = open_request_copy.clone();
            let resolve = move || {
                resolve
                    .call1(&resolve_request, &resolve_request.result().unwrap())
                    .unwrap();
            };
            let resolve = Closure::wrap(Box::new(resolve) as Box<dyn FnMut()>);

            let blocked_request = open_request_copy.clone();
            let reject_copy = reject.clone();
            let blocked = move || {
                reject_copy
                    .call1(&blocked_request, &blocked_request.error().unwrap().unwrap())
                    .unwrap();
            };
            let blocked = Closure::wrap(Box::new(blocked) as Box<dyn FnMut()>);

            let reject_request = open_request_copy.clone();
            let reject = move || {
                reject
                    .call1(&reject_request, &reject_request.error().unwrap().unwrap())
                    .unwrap();
            };
            let reject = Closure::wrap(Box::new(reject) as Box<dyn FnMut()>);

            open_request_copy.set_onsuccess(Some(&resolve.into_js_value().dyn_into().unwrap()));
            open_request_copy.set_onerror(Some(&reject.into_js_value().dyn_into().unwrap()));
            open_request_copy.set_onblocked(Some(&blocked.into_js_value().dyn_into().unwrap()));
        };
        let promise = Promise::new(&mut promise);

        // Create closure for "upgrade needed" event, which will only be fired when
        // initially creating the database.
        let open_request_copy = open_request.clone();
        let on_upgrade_needed = move || {
            // Just create a default object store. We are using the database as a simple
            // key-value store.
            let db: web_sys::IdbDatabase = open_request_copy.result().unwrap().dyn_into().unwrap();
            db.create_object_store("storage").unwrap();
        };
        let on_upgrade_needed = Closure::wrap(Box::new(on_upgrade_needed) as Box<dyn FnMut()>);
        open_request
            .set_onupgradeneeded(Some(&on_upgrade_needed.into_js_value().dyn_into().unwrap()));

        // Wait for the database to finish opening
        let db = JsFuture::from(promise)
            .await
            .map_err(|_| anyhow!("Failed to open IndexedDB"))?
            .dyn_into()
            .unwrap();

        Ok(Self { db })
    }

    pub async fn get(&self, key: &str) -> Result<Option<Vec<u8>>> {
        let transaction = self
            .db
            .transaction_with_str_and_mode("storage", IdbTransactionMode::Readonly)
            .map_err(|_| anyhow!("Failed to start IndexedDB transaction"))?;
        let store = transaction
            .object_store("storage")
            .map_err(|_| anyhow!("Transaction does not have object store"))?;
        let request = store
            .get(&JsValue::from_str(key))
            .map_err(|_| anyhow!("Failed to request database read"))?;

        // Create a `Promise` object for the request so that we can obtain a future
        let request_copy = request.clone();
        let mut promise = move |resolve: Function, reject: Function| {
            let resolve_request = request_copy.clone();
            let resolve = move || {
                resolve
                    .call1(&resolve_request, &resolve_request.result().unwrap())
                    .unwrap();
            };
            let resolve = Closure::wrap(Box::new(resolve) as Box<dyn FnMut()>);

            let reject_request = request_copy.clone();
            let reject = move || {
                reject
                    .call1(&reject_request, &reject_request.error().unwrap().unwrap())
                    .unwrap();
            };
            let reject = Closure::wrap(Box::new(reject) as Box<dyn FnMut()>);

            request_copy.set_onsuccess(Some(&resolve.into_js_value().dyn_into().unwrap()));
            request_copy.set_onerror(Some(&reject.into_js_value().dyn_into().unwrap()));
        };
        let promise = Promise::new(&mut promise);

        // Wait for the read to finish and return None if the key can't be read
        // (doesn't exist, invalid format, etc.)
        let value: Uint8Array = match JsFuture::from(promise).await {
            Ok(value) => {
                let value = value.dyn_into();
                match value {
                    Ok(value) => value,
                    Err(_) => return Ok(None),
                }
            }
            Err(_) => return Err(anyhow!("Read from database failed")),
        };

        // Convert `Uint8Array` to Rust `Vec` and return contents to caller
        let mut result = Vec::with_capacity(value.length() as usize);
        result.resize(value.length() as usize, 0);
        value.copy_to(&mut result[..]);
        Ok(Some(result))
    }

    pub async fn put(&mut self, key: &str, value: &[u8]) -> Result<()> {
        let transaction = self
            .db
            .transaction_with_str_and_mode("storage", IdbTransactionMode::Readwrite)
            .map_err(|_| anyhow!("Failed to start IndexedDB transaction"))?;
        let store = transaction
            .object_store("storage")
            .map_err(|_| anyhow!("Transaction does not have object store"))?;
        let value_buffer = Uint8Array::new_with_length(value.len() as u32);
        value_buffer.copy_from(value);
        let request = store
            .put_with_key(&value_buffer, &JsValue::from_str(key))
            .map_err(|_| anyhow!("Failed to request database write"))?;

        // Create a `Promise` object for the request so that we can obtain a future
        let request_copy = request.clone();
        let mut promise = move |resolve: Function, reject: Function| {
            let resolve_request = request_copy.clone();
            let resolve = move || {
                resolve.call0(&resolve_request).unwrap();
            };
            let resolve = Closure::wrap(Box::new(resolve) as Box<dyn FnMut()>);

            let reject_request = request_copy.clone();
            let reject = move || {
                reject
                    .call1(&reject_request, &reject_request.error().unwrap().unwrap())
                    .unwrap();
            };
            let reject = Closure::wrap(Box::new(reject) as Box<dyn FnMut()>);

            request_copy.set_onsuccess(Some(&resolve.into_js_value().dyn_into().unwrap()));
            request_copy.set_onerror(Some(&reject.into_js_value().dyn_into().unwrap()));
        };
        let promise = Promise::new(&mut promise);

        // Wait for the write to finish
        JsFuture::from(promise)
            .await
            .map_err(|_| anyhow!("Write to database failed"))?;

        Ok(())
    }

    pub async fn delete(&mut self, key: &str) -> Result<()> {
        let transaction = self
            .db
            .transaction_with_str_and_mode("storage", IdbTransactionMode::Readwrite)
            .map_err(|_| anyhow!("Failed to start IndexedDB transaction"))?;
        let store = transaction
            .object_store("storage")
            .map_err(|_| anyhow!("Transaction does not have object store"))?;
        let request = store
            .delete(&JsValue::from_str(key))
            .map_err(|_| anyhow!("Failed to request database item delete"))?;

        // Create a `Promise` object for the request so that we can obtain a future
        let request_copy = request.clone();
        let mut promise = move |resolve: Function, reject: Function| {
            let resolve_request = request_copy.clone();
            let resolve = move || {
                resolve.call0(&resolve_request).unwrap();
            };
            let resolve = Closure::wrap(Box::new(resolve) as Box<dyn FnMut()>);

            let reject_request = request_copy.clone();
            let reject = move || {
                reject
                    .call1(&reject_request, &reject_request.error().unwrap().unwrap())
                    .unwrap();
            };
            let reject = Closure::wrap(Box::new(reject) as Box<dyn FnMut()>);

            request_copy.set_onsuccess(Some(&resolve.into_js_value().dyn_into().unwrap()));
            request_copy.set_onerror(Some(&reject.into_js_value().dyn_into().unwrap()));
        };
        let promise = Promise::new(&mut promise);

        // Wait for the delete to finish
        JsFuture::from(promise)
            .await
            .map_err(|_| anyhow!("Delete item from database failed"))?;

        Ok(())
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
