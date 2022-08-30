use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum GatewayError {
    InvalidOpCode
}

impl Display for GatewayError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        use GatewayError::*;

        match self {
            InvalidOpCode => Display::fmt("Invalid OP code", f)
        }
    }
}