use std::{time::Duration, sync::{Arc, Mutex}, ops::DerefMut};

use futures::{StreamExt, TryStreamExt, stream::{MapErr, SplitSink}, SinkExt};
use tokio::{net::TcpStream, time};
use tokio_tungstenite::{connect_async, WebSocketStream, MaybeTlsStream, tungstenite::Message};

use crate::{json, Error, Result, Client};

use super::{Payload, GatewayOp, GatewayError};

// Hell no.
type Sink<S> = SplitSink<MapErr<WebSocketStream<MaybeTlsStream<TcpStream>>, S>, Message>;

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

    async fn handle_payload<S: 'static + Send>(&self, payload: Payload, sink: &Arc<Mutex<Sink<S>>>) -> Result<()> {
        println!("{payload:?}");
        let op = GatewayOp::from_code(payload.op);

        if let Some(op) = op {
            use GatewayOp::*;

            match op {
                Hello => {
                    let interval = payload.d["heartbeat_interval"].as_i64().unwrap() as u64;
                    let mut interval = time::interval(Duration::from_millis(interval));
                    
                    let sh_sink = Arc::clone(sink);
                    
                    tokio::spawn(async move {
                        loop {
                            interval.tick().await;
                            send_heartbeat(sh_sink).await;
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

async fn send_heartbeat<S>(sink: Arc<Mutex<Sink<S>>>) {
    println!("Sending heartbeat.");
}

async fn authorize_client<S>(sink: &Sink<S>) -> Result<()> {
    println!("Authorizing client.");
    Ok(())
}