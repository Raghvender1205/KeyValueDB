use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct KVPair {
    pub key: String,
    pub value: String,
    pub expires_at: Option<u64>,
}