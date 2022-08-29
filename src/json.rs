use serde::de::DeserializeOwned;

use crate::Result;

pub(crate) fn from_str<T: DeserializeOwned>(data: &str) -> Result<T> {
    Ok(serde_json::from_str(data)?)
}