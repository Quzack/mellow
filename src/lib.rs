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

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test() {
        let mut c = crate::Client::from_token("");

        c.on_event::<crate::event::Ready>(|_, _| {
            println!("No way");
        });

        c.start().await.unwrap();
    }
}