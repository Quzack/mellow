use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum Error {
    InvalidOpCode,
    UnknownEvent(String),
    InvalidSession
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result {
        use Error::*;

        match self {
            InvalidOpCode   => Display::fmt("Invalid OP code", f),
            UnknownEvent(e) => Display::fmt(&format!("Unknown event: {e}"), f),
            InvalidSession  => Display::fmt("Invalid session", f)
        }
    }
}