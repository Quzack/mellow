use futures::StreamExt;
use tokio_tungstenite::connect_async;

use crate::{json, Error, Result};

use super::{Payload, GatewayOp, GatewayError};

const DISCORD_GATEWAY_URL: &str = "wss://gateway.discord.gg/?v=10&encoding=json";

pub struct DiscordWsClient<'a> {
    pub token: &'a str,
    pub intents: &'a u16
}

impl<'a> DiscordWsClient<'a> {
    pub async fn open_connection(&self) -> Result<()> {
        let url = url::Url::parse(DISCORD_GATEWAY_URL).unwrap();
        let (stream, _) = connect_async(url).await?;

        let (_, read) = stream.split();

        read.for_each(|m| async {
            let data = m.unwrap().into_data();
            let payload: Payload = json::from_str(&String::from_utf8(data).unwrap()).unwrap();

            self.handle_payload(payload).await;
        }).await;

        Ok(())
    }

    async fn handle_payload(&self, payload: Payload) -> Result<()> {
        let op_code = GatewayOp::from_code(payload.op);

        if let Some(op) = op_code {
            // TODO: The thing.
        } else {
            return Err(Error::Gateway(GatewayError::InvalidOpCode(payload.op)))
        }

        Ok(())
    }
}