use std::{time::Duration, sync::Arc};

use futures::{StreamExt, TryStreamExt, stream::{MapErr, SplitSink}};
use tokio::{net::TcpStream, time};
use tokio_tungstenite::{connect_async, WebSocketStream, MaybeTlsStream, tungstenite::Message};

use crate::{json, Error, Result};

use super::{Payload, GatewayOp, GatewayError};

// Hell no.
type Sink<S> = SplitSink<MapErr<WebSocketStream<MaybeTlsStream<TcpStream>>, S>, Message>;

const DISCORD_GATEWAY_URL: &str = "wss://gateway.discord.gg/?v=10&encoding=json";

pub struct DiscordWsClient<'a> {
    pub token: &'a str,
    pub intents: &'a u16
}

impl<'a> DiscordWsClient<'a> {
    pub async fn open_connection(&self) -> Result<()> {
        let url = url::Url::parse(DISCORD_GATEWAY_URL).unwrap();
        let (t_stream, _) = connect_async(url).await.unwrap();

        let ws_stream = t_stream.map_err(|e| Error::Tungstenite(e));
        let (sink, stream) = ws_stream.split();

        let sink_sh = Arc::new(sink);

        stream.try_for_each(|m| async {
            let data = m.into_data();
            let payload: Payload = json::from_str(&String::from_utf8(data).unwrap()).unwrap();

            self.handle_payload(payload, &sink_sh).await
        }).await?;
        
        Ok(())
    }

    async fn handle_payload<S: 'static + Send>(&self, payload: Payload, sink: &Arc<Sink<S>>) -> Result<()> {
        println!("{payload:?}");
        let op = GatewayOp::from_code(payload.op);

        if let Some(op) = op {
            use GatewayOp::*;

            match op {
                Hello => {
                    let interval = payload.d["heartbeat_interval"].as_i64().unwrap() as u64;
                    let mut interval = time::interval(Duration::from_millis(interval));
                    
                    let sh_sink = sink.clone();

                    tokio::spawn(async move {
                        loop {
                            interval.tick().await;
                            send_heartbeat(sh_sink.as_ref()).await.unwrap();
                        }
                    });
                },
                Dispatch => {
                    // TODO: Packet handling....
                }
                _ => return Ok(())
            }
        } else {
            return Err(Error::Gateway(GatewayError::InvalidOpCode))
        }

        Ok(())
    }
}

async fn send_heartbeat<S>(sink: &Sink<S>) -> Result<()> {
    // TODO: Implementation...
    Ok(())
}