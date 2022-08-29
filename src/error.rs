use std::fmt::{Result, Display, Formatter};

use serde_json::Error as JsonError;

use crate::gateway::GatewayError;

#[derive(Debug)]
pub enum Error {
    Json(JsonError),
    Gateway(GatewayError)
}

impl From<JsonError> for Error {
    fn from(e: JsonError) -> Self {
        Error::Json(e)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result {
        use self::Error::*;

        match self {
            Json(e) => Display::fmt(&e, f),
            Gateway(e) => Display::fmt(&e, f)
        }
    }
}