use serde_json::Value;

use crate::{event::EventType, Result, Client};

mod client;

pub struct PacketRegistry {
    packet_handlers: Vec<Box<dyn PacketHandler>>
}

impl PacketRegistry {
    pub fn new() -> Self {
        Self {
            packet_handlers: vec![
                Box::new(client::ReadyHandler)
            ]
        }
    }

    pub fn handler_from_et(&self, et: EventType) -> Option<&Box<dyn PacketHandler>> {
        self.packet_handlers.iter().find(|h| h.ty() == et)
    }
}

pub trait PacketHandler {
    fn handle(&self, client: &mut Client, data: Value) -> Result<()>;

    fn ty(&self) -> EventType;
}