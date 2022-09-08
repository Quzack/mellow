use crate::{event::EventType, Result, Client};

mod ready;

pub struct PacketRegistry {
    packet_handlers: Vec<Box<dyn PacketHandler>>
}

impl PacketRegistry {
    pub fn new() -> Self {
        Self {
            packet_handlers: vec![
                Box::new(ready::ReadyHandler)
            ]
        }
    }

    pub fn handler_from_et(&self, et: EventType) -> Option<&Box<dyn PacketHandler>> {
        for handler in self.packet_handlers.iter() {
            if handler.ty() == et {
                return Some(handler);
            }
        }
        None
    }
}

pub trait PacketHandler {
    fn handle(&self, client: &mut Client, data: serde_json::Value) -> Result<()>;

    fn ty(&self) -> EventType;
}