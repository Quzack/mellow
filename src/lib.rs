mod client;
mod gateway;

pub(crate) type Result<T> = std::result::Result<T, crate::Error>;

pub(crate) mod json;
pub(crate) mod handler;

pub mod error;
pub mod model;
pub mod event;

pub use crate::{
    client::Client,
    error::Error
};

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn t() {
        let c = crate::Client::from_token("");

        c.start().await.unwrap();
    }
}