use serde::Deserialize;
use serde_json::Value;
use serde_repr::{Deserialize_repr, Serialize_repr};

mod client;
mod heart;

pub(crate) mod packet;

#[derive(Serialize_repr, Deserialize_repr, Debug)]
#[repr(u8)]
pub(crate) enum GatewayOp {
    Dispatch         = 0,
    Heartbeat        = 1,
    Identify         = 2,
    PresenceUpdate   = 3,
    VoiceStateUpdate = 4,
    Resume           = 6,
    Reconnect        = 7,
    ReqGuildMember   = 8,
    InvalidSession   = 9,
    Hello            = 10,
    HeartbeatRes     = 11
}

impl GatewayOp {
    pub fn code(&self) -> u8 {
        serde_json::to_string(self).unwrap().parse().unwrap()
    }
}

#[derive(Deserialize, Debug)]
pub(crate) struct Payload {
    pub op: GatewayOp,
    pub d:  Option<Value>,
    pub t:  Option<String>
}

pub mod error;

pub use self::{
    client::DiscordWsClient as WsClient,
    error::GatewayError as Error
};