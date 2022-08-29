mod ws_client;
mod payload;
mod gateway_op;

pub(crate) use self::{
    payload::Payload,
    gateway_op::GatewayOp
};

pub mod error;

pub use self::{
    ws_client::DiscordWsClient,
    error::GatewayError
};