use std::{sync::{Arc, Mutex}};

use futures::{StreamExt, TryStreamExt, SinkExt};
use serde_json::json;
use tokio::{sync::mpsc::{self, Sender}};
use tokio_tungstenite::tungstenite::Message;

use crate::{json, Error, Result, Client, event::EventType, gateway::{heart, packet, self}};

use super::{Payload, GatewayOp};

const DISCORD_GATEWAY_URL: &str = "wss://gateway.discord.gg/?v=10&encoding=json";

pub struct DiscordWsClient<'a> {
    sh_client:  Arc<Mutex<Client<'a>>>,
    sender:     Option<Sender<Message>>
}

impl<'a> DiscordWsClient<'a> {
    pub fn new(client: Client<'a>) -> DiscordWsClient<'a> {
        Self {
            sh_client:  Arc::new(Mutex::new(client)),
            sender:     None
        }
    }

    pub async fn connect(&mut self) -> Result<()> {
        let url = url::Url::parse(DISCORD_GATEWAY_URL).unwrap();
        let (t_stream, _) = tokio_tungstenite::connect_async(url).await.unwrap();

        let ws_stream = t_stream.map_err(|e| Error::Tungstenite(e));
        let (mut sink, stream) = ws_stream.split();
        
        let (sender, mut receiver) = mpsc::channel(100);

        self.sender = Some(sender);

        tokio::spawn(async move {
            while let Some(msg) = receiver.recv().await {
                sink.send(msg).await.expect("Failed to aggregate message to socket.");
            }
        });
        
        stream.try_for_each(|m| async {
            let payload: Payload = json::from_str(&m.to_string()).unwrap();
            drop(m);

             self.handle_payload(payload).await
        }).await?;
        
        Ok(())
    }

    async fn handle_payload(&self, payload: Payload) -> Result<()> {
        let sender = self.sender.as_ref().unwrap();

        let mtg_client = self.sh_client.clone();
        let mut client = mtg_client.lock().unwrap();

        use GatewayOp::*;

        match payload.op {
            Hello => {
                let interval = payload.d.unwrap()["heartbeat_interval"].as_i64().unwrap();
                
                heart::start_heart(interval as u64, &sender).await;
                auth_client(sender, &client).await;
            },
            Dispatch => {
                let e_name = payload.t.unwrap();

                let et = match EventType::from_str(&e_name) {
                    Some(ty) => ty,
                    None     => return Err(Error::Gateway(gateway::Error::UnknownEvent(e_name)))
                };

                packet::handler_from_et(et).handle(&mut client, payload.d.unwrap())?;
            },
            InvalidSession => return Err(Error::Gateway(gateway::Error::InvalidSession)),
            _              => return Ok(())
        }

        Ok(())
    }
}

async fn auth_client<'a>(sender: &Sender<Message>, client: &Client<'a>) {
    let auth_json = json!({
        "op": GatewayOp::Identify.code(),
        "d": {
            "token": client.token,
            "intents": client.intents,
            "properties": {
                "os": "linux",
                "browser": "mellow",
                "device": "mellow"
            }
        }
    }).to_string();

    sender.send(Message::Text(auth_json)).await.expect("Failed to authenticate client");
}