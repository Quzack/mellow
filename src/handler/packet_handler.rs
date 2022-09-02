use serde_json::Value;

use crate::{Result, Client};

pub trait PacketHandler {
    fn handle(data: Value, client: &Client) -> Result<()>;
}