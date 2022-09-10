use serde_json::Value;

use crate::{event::{EventType, Ready}, Client, json, Result};

use super::PacketHandler;

pub struct ReadyHandler;

impl PacketHandler for ReadyHandler {
    fn handle(&self, client: &mut Client, data: Value) -> Result<()> {
        client.user = json::from_str(&data["user"].to_string())?;
        client.emit_event(Ready);
        
        Ok(())
    }

    fn ty(&self) -> EventType {
       EventType::Ready 
    }
}