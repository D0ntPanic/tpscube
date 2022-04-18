use crate::sync::TABLE_NAME;
use anyhow::Result;
use rusoto_core::RusotoError;
use rusoto_dynamodb::{AttributeValue, DynamoDb, DynamoDbClient, PutItemError, PutItemInput};
use std::collections::HashMap;
use tpscube_core::StoredAction;

// There is a 400kb maximum item size on the database, so keep a reasonable maximum number of
// actions to store in a single item. This prevents errors on syncing a very large set of
// actions (such as after a large import).
const MAX_ACTIONS_PER_ITEM: usize = 64;

pub async fn store_actions(
    client: &DynamoDbClient,
    sync_key: &str,
    sync_id: u32,
    actions: &[StoredAction],
) -> Result<(u32, usize)> {
    if actions.len() == 0 {
        // Don't store empty lists in the database
        return Ok((sync_id, 0));
    }

    // Sync ID for the freshly added actions is one more than the previous sync point. To prevent
    // race conditions it will be verified that no data already exists at that ID.
    let mut committed_sync_id = sync_id;
    let mut new_sync_id = sync_id + 1;
    let mut written_count = 0;

    // Write actions in chunks to avoid running into the item size limit
    for chunk in actions.chunks(MAX_ACTIONS_PER_ITEM) {
        let data = zstd::encode_all(StoredAction::serialize_list(chunk).as_slice(), 0)?;

        let mut item = HashMap::new();
        item.insert(
            "sync_key".into(),
            AttributeValue {
                s: Some(sync_key.into()),
                ..Default::default()
            },
        );
        item.insert(
            "sync_id".into(),
            AttributeValue {
                n: Some(format!("{}", new_sync_id)),
                ..Default::default()
            },
        );
        item.insert(
            "data".into(),
            AttributeValue {
                b: Some(data.into()),
                ..Default::default()
            },
        );

        let put = PutItemInput {
            table_name: TABLE_NAME.into(),
            condition_expression: Some("attribute_not_exists(sync_id)".into()),
            item,
            ..Default::default()
        };
        match client.put_item(put).await {
            Ok(result) => result,
            Err(RusotoError::Service(PutItemError::ConditionalCheckFailed(_))) => {
                // There was already an item at the sync ID slot requested. This means that another client
                // has uploaded actions since the previous query. Don't let the write complete and tell
                // the client to try again. Still report any partial progress so that the client knows
                // to resubmit only the incomplete parts.
                return Ok((committed_sync_id, written_count));
            }
            Err(error) => return Err(error.into()),
        };

        // Actions were written to the database, mark the newly written sync ID as committed and
        // report to the client that these actions are complete.
        committed_sync_id = new_sync_id;
        written_count += chunk.len();

        // If there are any more chunks, they need to be written to the next sync ID
        new_sync_id += 1;
    }

    return Ok((committed_sync_id, written_count));
}
