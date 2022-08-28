use std::fmt::{Result, Display, Formatter};

#[derive(Debug)]
pub enum Error {
    // TODO: Enumerate errors.
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result {
        Ok(())
    }
}