use serde::Deserialize;
use serde_json::Value;
use serde_repr::{Deserialize_repr, Serialize_repr};

mod client;

pub(crate) mod packet;

pub(crate) const GATEWAY_URL: &str = "wss://gateway.discord.gg/?v=10&encoding=json";
pub(crate) const GATEWAY_VER: u8 = 10;

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
    client::Client as WsClient,
    error::Error
};