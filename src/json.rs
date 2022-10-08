use serde::de::DeserializeOwned;
use serde_json::Value;

use crate::Result;

macro_rules! json_mt {
    ($($json:tt)+) => {
        tokio_tungstenite::tungstenite::Message::Text(serde_json::json!($($json)+).to_string())
    };
}

pub(crate) use json_mt;

pub(crate) fn from_str<T: DeserializeOwned>(data: &str) -> Result<T> {
    Ok(serde_json::from_str(data)?)
}

pub(crate) fn from_val<T: DeserializeOwned>(data: &Value) -> Result<T> {
    Ok(from_str(&data.to_string())?)
}