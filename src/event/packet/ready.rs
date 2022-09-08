use crate::{event::EventType, Client, json, Result};

use super::PacketHandler;

pub struct ReadyHandler;

impl PacketHandler for ReadyHandler {
    fn handle(&self, client: &mut Client, data: serde_json::Value) -> Result<()> {
        client.user = json::from_str(&data["user"].to_string())?;
        client.emit_event(crate::event::Ready);
        
        Ok(())
    }

    fn ty(&self) -> EventType {
       EventType::Ready 
    }
}