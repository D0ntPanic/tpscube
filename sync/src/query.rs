use crate::sync::TABLE_NAME;
use anyhow::anyhow;
use anyhow::Result;
use rusoto_dynamodb::{AttributeValue, DynamoDb, DynamoDbClient, QueryInput};
use std::collections::HashMap;
use tpscube_core::StoredAction;

pub struct Updates {
    pub sync_id: u32,
    pub actions: Vec<StoredAction>,
    pub more_actions: bool,
}

pub async fn query_updates(
    client: &DynamoDbClient,
    sync_key: &str,
    sync_id: u32,
) -> Result<Updates> {
    // Query the database to find all actions after the client's sync point.
    let mut actions = Vec::new();
    let mut last_sync_id = sync_id;
    let mut values = HashMap::new();
    values.insert(
        ":key".into(),
        AttributeValue {
            s: Some(sync_key.into()),
            ..Default::default()
        },
    );
    values.insert(
        ":id".into(),
        AttributeValue {
            n: Some(format!("{}", sync_id)),
            ..Default::default()
        },
    );
    let query = QueryInput {
        table_name: TABLE_NAME.into(),
        key_condition_expression: Some("sync_key = :key AND sync_id > :id".into()),
        expression_attribute_values: Some(values),
        ..Default::default()
    };
    let result = client.query(query).await?;

    // Decode and add results to the query result list
    if let Some(items) = result.items {
        for item in items {
            last_sync_id = item
                .get("sync_id")
                .ok_or_else(|| anyhow!("Missing sync ID in query result"))?
                .n
                .as_ref()
                .ok_or_else(|| anyhow!("Sync ID is not a number in query result"))?
                .parse()?;
            if let Some(data) = item.get("data") {
                if let Some(binary) = &data.b {
                    let mut new_actions =
                        StoredAction::deserialize_list(&zstd::decode_all(binary.as_ref())?)?;
                    actions.append(&mut new_actions);
                }
            }
        }
    }

    // Check to see if there are more results. If there are, tell the client so that it can issue
    // another request for the rest of the actions. We let the client perform multiple requests so
    // that a very large sync will not time out in Lambda.
    let more_actions = result.last_evaluated_key.is_some();

    Ok(Updates {
        sync_id: last_sync_id,
        actions,
        more_actions,
    })
}
