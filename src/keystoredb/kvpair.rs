use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct KVPair {
    pub key: String,
    pub value: String,
}