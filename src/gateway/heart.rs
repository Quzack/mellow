use serde_json::json;
use tokio::sync::mpsc::Sender;
use tokio_tungstenite::tungstenite::Message;
use std::time::Duration;

use super::GatewayOp;

pub async fn start_heart(timer: u64, sender: &Sender<Message>) {
    let sender = sender.clone();

    let mut interval = tokio::time::interval(Duration::from_millis(timer));

    tokio::spawn(async move {
        loop {
            interval.tick().await;
            
            pulse(&sender).await;
        }
    });
}

async fn pulse(sender: &Sender<Message>) {
    let heartbeat = json!({
        "op": GatewayOp::Heartbeat.code(),
        "d": "null"
    }).to_string();
    
    sender.send(Message::Text(heartbeat)).await.unwrap();
}