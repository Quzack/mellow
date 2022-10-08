use std::{sync::{Arc, Mutex}};

use futures::{StreamExt, TryStreamExt, SinkExt};
use tokio::{sync::mpsc::{self, Sender}};
use tokio_tungstenite::tungstenite::Message;

use crate::{json, Error, event::EventType, gateway::{packet, self}};

use super::{Payload, GatewayOp};

pub struct Client<'a> {
    sh_client:  Arc<Mutex<crate::Client<'a>>>,
    sender:     Option<Sender<Message>>
}

impl<'a> Client<'a> {
    pub fn new(client: crate::Client<'a>) -> Client<'a> {
        Self {
            sh_client:  Arc::new(Mutex::new(client)),
            sender:     None
        }
    }

    pub async fn connect(&mut self) -> crate::Result<()> {
        let url = url::Url::parse(super::GATEWAY_URL).unwrap();
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

    async fn handle_payload(&self, payload: Payload) -> crate::Result<()> {
        let mtg_client = self.sh_client.clone();
        let mut client = mtg_client.lock().unwrap();

        use GatewayOp::*;

        match payload.op {
            Hello => {
                let interval = payload.d.unwrap()["heartbeat_interval"].as_i64().unwrap();
                self.start_heart(interval as u64).await;

                self.send_identify(client.get_token(), client.get_intents()).await;
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
    
    async fn send_identify(
        &self,
        token:   &str,
        intents: &u16
    ) {
        self.sender.as_ref().unwrap().clone().send(json::json_mt!(
            {
                "op": GatewayOp::Identify.code(),
                "d": {
                    "token": token,
                    "intents": intents,
                    "v": super::GATEWAY_VER,
                    "properties": {
                        "$os": "linux",
                        "$browser": "mellow",
                        "$device": "mellow"
                    }
                }
            }
        )).await.unwrap();
    }

    async fn start_heart(&self, timer: u64) {
        let mut interval = tokio::time::interval(std::time::Duration::from_millis(timer));
        let sender = self.sender.as_ref().unwrap().clone();

        tokio::spawn(async move {
            loop {
                interval.tick().await;
                
                sender.send(json::json_mt!(
                    {
                        "op": GatewayOp::Heartbeat.code(),
                        "d": "null"
                    }
                )).await.unwrap();
            }
        });
    }
}