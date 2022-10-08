mod client;

pub mod error;

pub use self::{
    client::Client as HttpClient,
    error::Error
};