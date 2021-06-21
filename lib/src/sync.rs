use crate::request::{SyncRequest, SyncResponse};
use anyhow::{anyhow, Result};
use serde_json::Value;
use std::sync::{Arc, Mutex};

#[cfg(feature = "native-storage")]
use http::StatusCode;
#[cfg(feature = "native-storage")]
use reqwest::{
    blocking::Client,
    header::{HeaderValue, CONTENT_TYPE, USER_AGENT},
};

#[cfg(feature = "web-storage")]
use wasm_bindgen::JsValue;

const ENDPOINT: &'static str = "https://api.tpscube.xyz/sync";

pub(crate) struct SyncOperation {
    request: SyncRequest,
    response: Option<Result<SyncResponse>>,
}

#[derive(Clone)]
pub enum SyncStatus {
    NotSynced,
    SyncPending,
    SyncFailed(String),
    SyncComplete,
}

#[cfg(feature = "web-storage")]
fn spawn_future<F>(future: F)
where
    F: std::future::Future<Output = ()> + 'static,
{
    wasm_bindgen_futures::spawn_local(future);
}

impl SyncOperation {
    pub fn new(request: SyncRequest) -> Arc<Mutex<Self>> {
        let operation = Arc::new(Mutex::new(Self {
            request,
            response: None,
        }));

        let operation_copy = operation.clone();

        #[cfg(feature = "native-storage")]
        std::thread::spawn(move || {
            let result = Self::execute(&operation_copy);
            operation_copy.lock().unwrap().response = Some(result);
        });

        #[cfg(feature = "web-storage")]
        spawn_future(async move {
            let result = Self::execute(&operation_copy).await;
            operation_copy.lock().unwrap().response = Some(result);
        });

        operation
    }

    #[cfg(feature = "native-storage")]
    fn execute_native(request: String) -> Result<SyncResponse> {
        let client = Client::new();
        let result = client
            .post(ENDPOINT)
            .header(USER_AGENT, HeaderValue::from_static("tpscube"))
            .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
            .body(request)
            .send()?;

        // Check status code
        if result.status().is_success() {
            // Request success, deserialize as sync response
            SyncResponse::deserialize(result.json()?)
        } else if result.status() == StatusCode::BAD_REQUEST {
            // Bad request status contains a message from the server, decode it and pass
            // it along as the error.
            let message_json: Value = result.json()?;
            let message = message_json
                .get("message")
                .ok_or_else(|| anyhow!("Bad request"))?
                .as_str()
                .ok_or_else(|| anyhow!("Bad request"))?;
            Err(anyhow!("{}", message))
        } else {
            // For other status codes, use the standard HTTP reasons as the error
            Err(anyhow!(
                "{}",
                result
                    .status()
                    .canonical_reason()
                    .ok_or_else(|| anyhow!("Request failed"))?
            ))
        }
    }

    #[cfg(feature = "web-storage")]
    async fn execute_web(request: String) -> Result<SyncResponse> {
        use wasm_bindgen::JsCast;
        use wasm_bindgen_futures::JsFuture;

        let mut init = web_sys::RequestInit::new();
        init.method("POST");
        init.mode(web_sys::RequestMode::Cors);
        init.body(Some(&JsValue::from_str(&request)));

        let request = web_sys::Request::new_with_str_and_init(ENDPOINT, &init)
            .map_err(|_| anyhow!("Request init failed"))?;
        request
            .headers()
            .set("Content-Type", "text/plain")
            .map_err(|_| anyhow!("Failed to set headers"))?;

        let window = web_sys::window().unwrap();
        let response = JsFuture::from(window.fetch_with_request(&request))
            .await
            .map_err(|_| anyhow!("Fetch failed"))?;
        assert!(response.is_instance_of::<web_sys::Response>());

        let response: web_sys::Response = response.dyn_into().unwrap();
        let array_buffer = JsFuture::from(
            response
                .array_buffer()
                .map_err(|_| anyhow!("Response could not be fetched as an array buffer"))?,
        )
        .await
        .map_err(|_| anyhow!("Response could not be fetched as an array buffer"))?;
        let uint8_array = js_sys::Uint8Array::new(&array_buffer);
        let bytes = uint8_array.to_vec();
        let result = String::from_utf8(bytes.clone()).map_err(|_| anyhow!("Bad response body"))?;

        // Check status code
        if response.status() >= 200 && response.status() <= 299 {
            // Request success, deserialize as sync response
            let result: Value = serde_json::from_str(&result)?;
            SyncResponse::deserialize(result)
        } else if response.status() == 400 {
            // Bad request status contains a message from the server, decode it and pass
            // it along as the error.
            let message_json: Value = serde_json::from_str(&result)?;
            let message = message_json
                .get("message")
                .ok_or_else(|| anyhow!("Bad request"))?
                .as_str()
                .ok_or_else(|| anyhow!("Bad request"))?;
            Err(anyhow!("{}", message))
        } else {
            // For other status codes, use the standard HTTP reasons as the error
            Err(anyhow!("{}", response.status_text()))
        }
    }

    #[cfg(feature = "native-storage")]
    fn execute(operation: &Arc<Mutex<Self>>) -> Result<SyncResponse> {
        // Serialize request and send response
        let request = operation.lock().unwrap().request.serialize()?.to_string();
        Self::execute_native(request)
    }

    #[cfg(feature = "web-storage")]
    async fn execute(operation: &Arc<Mutex<Self>>) -> Result<SyncResponse> {
        // Serialize request and send response
        let request = operation.lock().unwrap().request.serialize()?.to_string();
        Self::execute_web(request).await
    }

    pub fn done(&self) -> bool {
        self.response.is_some()
    }

    pub fn response(&self) -> &Option<Result<SyncResponse>> {
        &self.response
    }
}
