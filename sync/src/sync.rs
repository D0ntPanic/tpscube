use crate::query::query_updates;
use crate::store::store_actions;
use anyhow::{anyhow, Result};
use lambda_http::{http::StatusCode, lambda_runtime::Error, Body, Request, Response};
use rusoto_core::Region;
use rusoto_dynamodb::DynamoDbClient;
use serde_json::Value;
use tpscube_core::{SyncRequest, SyncResponse, SYNC_API_VERSION};

pub const TABLE_NAME: &'static str = "tpscube";

pub fn response(status: StatusCode, value: Value) -> Result<Response<Body>, Error> {
    Ok(Response::builder()
        .status(status)
        .header("Content-Type", "application/json")
        .header("Access-Control-Allow-Origin", "*")
        .body(Body::from(value.to_string()))?)
}

pub async fn perform_sync(request: Request) -> Result<Value> {
    // Deserialize request and connect to database
    let request = match request.body() {
        Body::Text(body) => body,
        _ => return Err(anyhow!("Request must be JSON text")),
    };
    let request: Value = serde_json::from_str(request)?;

    let api_version = request
        .get("api_version")
        .ok_or_else(|| anyhow!("Request did not supply API version"))?
        .as_u64()
        .ok_or_else(|| anyhow!("API version is not an integer"))?;
    if api_version != SYNC_API_VERSION {
        return Err(anyhow!("API version mismatch, please update the client"));
    }

    let request = SyncRequest::deserialize(request)?;
    let client = DynamoDbClient::new(Region::UsEast1);

    // Get any new updates based on client's last sync
    let updates = query_updates(&client, &request.sync_key, request.sync_id).await?;

    // Store any new actions from the client
    let (sync_id, uploaded) = match request.upload {
        Some(actions) => {
            if updates.actions.len() == 0 {
                // No new actions since last sync, OK to store client's new actions
                store_actions(&client, &request.sync_key, updates.sync_id, &actions).await?
            } else {
                // New actions have happened since last client sync, this upload is invalid. Let
                // the client resolve the new actions and request the upload again.
                (updates.sync_id, 0)
            }
        }
        None => (updates.sync_id, 0),
    };

    // Serialize response
    SyncResponse {
        new_sync_id: sync_id,
        new_actions: updates.actions,
        more_actions: updates.more_actions,
        uploaded,
    }
    .serialize()
}
