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

impl SyncOperation {
    pub fn new(request: SyncRequest) -> Arc<Mutex<Self>> {
        let operation = Arc::new(Mutex::new(Self {
            request,
            response: None,
        }));

        let operation_copy = operation.clone();
        std::thread::spawn(move || {
            let result = Self::execute(&operation_copy);
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
    fn execute_web(request: String) -> Result<SyncResponse> {
        Err(anyhow!("Unimplemented"))
    }

    fn execute(operation: &Arc<Mutex<Self>>) -> Result<SyncResponse> {
        // Serialize request and send response
        let request = operation.lock().unwrap().request.serialize()?.to_string();

        #[cfg(feature = "native-storage")]
        let result = Self::execute_native(request);
        #[cfg(feature = "web-storage")]
        let result = Self::execute_web(request);

        result
    }

    pub fn done(&self) -> bool {
        self.response.is_some()
    }

    pub fn response(&self) -> &Option<Result<SyncResponse>> {
        &self.response
    }
}
