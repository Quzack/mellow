use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum GatewayError {
    // TODO: Implementation.
}

impl Display for GatewayError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        Ok(())
    }
}