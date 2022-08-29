use std::fmt::{Result, Display, Formatter};

use serde_json::Error as JsonError;
use tokio_tungstenite::tungstenite::Error as TungteniteError;

use crate::gateway::GatewayError;

#[derive(Debug)]
pub enum Error {
    Json(JsonError),
    Gateway(GatewayError),
    Tungtenite(TungteniteError)
}

impl From<JsonError> for Error {
    fn from(e: JsonError) -> Self {
        Error::Json(e)
    }
}

impl From<TungteniteError> for Error {
    fn from(e: TungteniteError) -> Self {
        Error::Tungtenite(e)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result {
        use self::Error::*;

        match self {
            Json(e) => Display::fmt(&e, f),
            Gateway(e) => Display::fmt(&e, f),
            Tungtenite(e) => Display::fmt(&e, f)
        }
    }
}