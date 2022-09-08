use serde::Deserialize;
use serde_json::Value;

mod ws_client;
mod gateway_op;

pub(crate) use gateway_op::GatewayOp;

#[derive(Deserialize, Debug)]
pub(crate) struct Payload {
    pub op: u8,
    pub d:  Option<Value>,
    pub t:  Option<String>,
    pub s:  Option<usize>
}

pub mod error;

pub use self::{
    ws_client::DiscordWsClient as WsClient,
    error::GatewayError
};