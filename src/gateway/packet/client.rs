use serde_json::Value;

use crate::{event::{EventType, Ready}, Client, json, Result};

use super::PacketHandler;

pub struct ReadyHandler;

impl PacketHandler for ReadyHandler {
    fn handle(
        &self, 
        client: &mut Client, 
        data:   Value
    ) -> Result<()> {
        client.user = json::from_val(&data["user"])?;
        client.app = json::from_val(&data["application"])?;

        client.emit_event(Ready { guilds: json::from_val(&data["guilds"])? });
        
        Ok(())
    }

    fn ty(&self) -> EventType {
       EventType::Ready 
    }
}