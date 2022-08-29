mod ws_client;

pub mod error;

pub use self::{
    ws_client::DiscordWsClient,
    error::GatewayError
};