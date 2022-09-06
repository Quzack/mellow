use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize, Debug)]
pub struct Payload {
    pub op: u8,
    pub d: Option<Value>,
    pub t: Option<String>,
    pub s: Option<usize>
}