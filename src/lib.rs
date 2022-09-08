mod client;
mod gateway;

pub(crate) type Result<T> = std::result::Result<T, crate::Error>;

pub(crate) mod json;

pub mod error;
pub mod model;
pub mod event;

pub use crate::{
    client::Client,
    error::Error
};