use crate::action::StoredAction;
use anyhow::{anyhow, Result};
use rand::{thread_rng, Rng};
use serde_json::{json, Value};
use std::convert::TryInto;

pub const SYNC_API_VERSION: u64 = 1;

const SYNC_KEY_CHARS: [char; 32] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'C', 'D', 'E', 'F', 'G', 'H', 'J', 'K',
    'M', 'N', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];
const SYNC_KEY_LENGTH: usize = 20;
const SYNC_KEY_GROUPING: usize = 4;
const SYNC_KEY_VALIDATION_BITS: usize = 20;

#[derive(Clone, Debug)]
pub struct SyncRequest {
    pub sync_key: String,
    pub sync_id: u32,
    pub upload: Option<Vec<StoredAction>>,
}

#[derive(Clone, Debug)]
pub struct SyncResponse {
    pub new_sync_id: u32,
    pub new_actions: Vec<StoredAction>,
    pub more_actions: bool,
    pub uploaded: usize,
}

impl SyncRequest {
    pub fn new_sync_key() -> String {
        // Generate a random sync identifier
        let id_range =
            (SYNC_KEY_CHARS.len() as u128).pow(SYNC_KEY_LENGTH as u32) >> SYNC_KEY_VALIDATION_BITS;
        let id: u128 = thread_rng().gen_range(0..id_range);

        // Compute checksum digits for validation and combine into the sync key value
        let checksum = Self::sync_key_checksum(id);
        let mut value = (id << SYNC_KEY_VALIDATION_BITS) + checksum;

        // Generate sync key characters from its integer value (base 32)
        let mut chars = Vec::new();
        for _ in 0..SYNC_KEY_LENGTH {
            chars.push(SYNC_KEY_CHARS[(value % SYNC_KEY_CHARS.len() as u128) as usize]);
            value /= SYNC_KEY_CHARS.len() as u128;
        }
        chars.reverse();

        // Split characters into groups for visual aid
        let groups: Vec<String> = chars
            .as_slice()
            .chunks(SYNC_KEY_GROUPING)
            .map(|chunk| chunk.iter().collect())
            .collect();
        groups.join("-")
    }

    pub fn validate_sync_key(key: &str) -> Option<String> {
        // Unify sync key format by getting rid of whitespace and dashes, replacing commonly
        // mistaken characters, and forcing uppercase
        let key = key
            .trim()
            .to_uppercase()
            .replace("-", "")
            .replace("I", "1")
            .replace("O", "0");
        if key.len() != SYNC_KEY_LENGTH {
            return None;
        }

        // Compute integer value from sync key characters (base 32)
        let mut value: u128 = 0;
        for ch in key.chars() {
            match SYNC_KEY_CHARS.binary_search(&ch) {
                Ok(idx) => value = value * SYNC_KEY_CHARS.len() as u128 + idx as u128,
                _ => return None,
            };
        }

        // Split sync key value into sync key identifier and checksum
        let id = value >> SYNC_KEY_VALIDATION_BITS;
        let checksum = value & ((1 << SYNC_KEY_VALIDATION_BITS) - 1);

        // Validate checksum
        if checksum != Self::sync_key_checksum(id) {
            return None;
        }

        Some(key)
    }

    fn sync_key_checksum(id: u128) -> u128 {
        // Hash the bytes of the identifier with a 32 bit hash and return
        // the bottom bits as the validation checksum
        let mut value = id;
        let mut checksum: u32 = 0;
        for _ in 0..(128 / 8) {
            checksum = checksum.wrapping_add((value & 0xff) as u32);
            checksum = checksum.wrapping_add(checksum.wrapping_shl(10));
            checksum ^= checksum >> 6;
            value >>= 8;
        }
        checksum = checksum.wrapping_add(checksum.wrapping_shl(3));
        checksum ^= checksum >> 11;
        checksum = checksum.wrapping_add(checksum.wrapping_shl(15));
        (checksum & ((1 << SYNC_KEY_VALIDATION_BITS) - 1)) as u128
    }

    pub fn fetch(sync_key: String, sync_id: u32) -> Self {
        Self {
            sync_key,
            sync_id,
            upload: None,
        }
    }

    pub fn upload(sync_key: String, sync_id: u32, actions: Vec<StoredAction>) -> Self {
        Self {
            sync_key,
            sync_id,
            upload: Some(actions),
        }
    }

    pub fn serialize(&self) -> Result<Value> {
        Ok(match &self.upload {
            Some(upload) => {
                let upload = base64::encode(StoredAction::serialize_list(upload)?);
                json!({
                    "api_version": SYNC_API_VERSION,
                    "sync_key": self.sync_key,
                    "sync_id": self.sync_id,
                    "upload": upload
                })
            }
            None => {
                json!({
                    "api_version": SYNC_API_VERSION,
                    "sync_key": self.sync_key,
                    "sync_id": self.sync_id
                })
            }
        })
    }

    pub fn deserialize(request: Value) -> Result<Self> {
        let sync_key = Self::validate_sync_key(
            request
                .get("sync_key")
                .ok_or_else(|| anyhow!("Missing sync key"))?
                .as_str()
                .ok_or_else(|| anyhow!("Sync key is not a string"))?,
        )
        .ok_or_else(|| anyhow!("Invalid sync key"))?;

        let sync_id: u32 = request
            .get("sync_id")
            .ok_or_else(|| anyhow!("Missing sync ID"))?
            .as_u64()
            .ok_or_else(|| anyhow!("Sync ID is not an integer"))?
            .try_into()?;

        let upload = match request.get("upload") {
            Some(data) => Some(StoredAction::deserialize_list(&base64::decode(
                data.as_str()
                    .ok_or_else(|| anyhow!("Upload data is not a base64 string"))?,
            )?)?),
            None => None,
        };

        Ok(Self {
            sync_key,
            sync_id,
            upload,
        })
    }
}

impl SyncResponse {
    pub fn serialize(&self) -> Result<Value> {
        if self.new_actions.len() == 0 {
            Ok(json!({
                "sync_id": self.new_sync_id,
                "uploaded": self.uploaded
            }))
        } else {
            let new_data = base64::encode(StoredAction::serialize_list(&self.new_actions)?);
            Ok(json!({
                "sync_id": self.new_sync_id,
                "data": new_data,
                "more": self.more_actions,
                "uploaded": self.uploaded
            }))
        }
    }

    pub fn deserialize(response: Value) -> Result<Self> {
        let new_sync_id: u32 = response
            .get("sync_id")
            .ok_or_else(|| anyhow!("Missing sync ID"))?
            .as_u64()
            .ok_or_else(|| anyhow!("Sync ID is not an integer"))?
            .try_into()?;
        let new_actions = match response.get("data") {
            Some(data) => StoredAction::deserialize_list(&base64::decode(
                data.as_str()
                    .ok_or_else(|| anyhow!("Data is not a base64 string"))?,
            )?)?,
            None => Vec::new(),
        };
        let more_actions = match response.get("more") {
            Some(more) => more
                .as_bool()
                .ok_or_else(|| anyhow!("More actions flag is not a bool"))?,
            None => false,
        };
        let uploaded = response
            .get("uploaded")
            .ok_or_else(|| anyhow!("Missing upload count"))?
            .as_u64()
            .ok_or_else(|| anyhow!("Upload count is not an integer"))?
            .try_into()?;

        Ok(Self {
            new_sync_id,
            new_actions,
            more_actions,
            uploaded,
        })
    }
}
