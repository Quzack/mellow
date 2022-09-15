use serde_json::Value;

use crate::{event::{EventType, Ready}, Client, json, Result, model::guild::UnavailableGuild};

use super::PacketHandler;

pub struct ReadyHandler;

impl PacketHandler for ReadyHandler {
    fn handle(&self, client: &mut Client, data: Value) -> Result<()> {
        client.user        = json::from_val(&data["user"])?;
        client.session_id  = json::from_val(&data["session_id"])?;
        client.application = json::from_val(&data["application"])?;

        let guilds: Vec<UnavailableGuild> = json::from_val(&data["guilds"])?;
        client.emit_event(Ready { guilds });
        
        Ok(())
    }

    fn ty(&self) -> EventType {
       EventType::Ready 
    }
}