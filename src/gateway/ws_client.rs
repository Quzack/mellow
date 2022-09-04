use std::{time::Duration, sync::{Mutex, Arc}};

use futures::{StreamExt, TryStreamExt, Sink, SinkExt};
use serde_json::json;
use tokio::time;
use tokio_tungstenite::{connect_async, tungstenite::Message};

use crate::{json, Error, Result, Client};

use super::{Payload, GatewayOp, GatewayError};

const DISCORD_GATEWAY_URL: &str = "wss://gateway.discord.gg/?v=10&encoding=json";

pub struct DiscordWsClient<'a> {
    pub client: Client<'a>
}

impl<'a> DiscordWsClient<'a> {
    pub async fn open_connection(&self) -> Result<()> {
        let url = url::Url::parse(DISCORD_GATEWAY_URL).unwrap();
        let (t_stream, _) = connect_async(url).await.unwrap();

        let ws_stream = t_stream.map_err(|e| Error::Tungstenite(e));
        let (sink, stream) = ws_stream.split();

        let sink_sh = Arc::new(Mutex::new(sink));
        
        stream.try_for_each(|m| async {
            let data = m.into_data();
            let payload: Payload = json::from_str(&String::from_utf8(data).unwrap()).unwrap();

            self.handle_payload(payload, &sink_sh).await
        }).await?;
        
        Ok(())
    }

    async fn handle_payload<S>(&self, payload: Payload, sink: &Arc<Mutex<S>>) -> Result<()> 
    where
        S: Sink<Message> + Send + Unpin + 'static
    {
        println!("{payload:?}");
        let op = GatewayOp::from_code(payload.op);

        if let Some(op) = op {
            use GatewayOp::*;

            match op {
                Hello => {
                    let interval = payload.d["heartbeat_interval"].as_i64().unwrap() as u64;
                    let mut interval = time::interval(Duration::from_millis(interval));
                               
                    tokio::spawn(async move {
                        let sh_sink = Arc::clone(sink);

                        loop {
                            interval.tick().await;
                            send_heartbeat(&sh_sink).await;
                        }
                    });
                },
                Dispatch => {

                }
                _ => return Ok(())
            }
        } else {
            return Err(Error::Gateway(GatewayError::InvalidOpCode))
        }

        Ok(())
    }
}

async fn send_heartbeat(sink: &Arc<Mutex<impl Sink<Message> + Unpin>>) {
    let heartbeat = json!({
        "op": GatewayOp::Heartbeat.code()
    });

    sink.lock().unwrap().send(Message::Text(heartbeat.to_string())).await;
    println!("Sending heartbeat.");
}