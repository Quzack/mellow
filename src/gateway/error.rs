use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum GatewayError {
    InvalidOpCode(u8)
}

impl Display for GatewayError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        use GatewayError::*;

        match self {
            InvalidOpCode(c) => Display::fmt(&format!("Received invalid OP code: {}", c), f)
        }
    }
}