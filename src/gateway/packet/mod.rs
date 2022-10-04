use once_cell::sync::OnceCell;
use serde_json::Value;

use crate::{event::EventType, Result, Client};

mod client;

pub fn handler_from_et(e_ty: EventType) -> &'static Box<dyn PacketHandler> {
    static HANDLERS: OnceCell<Vec<Box<dyn PacketHandler>>> = OnceCell::new();

    HANDLERS.get_or_init(|| {
        vec![
            Box::new(client::ReadyHandler)
        ]
    }).iter().find(|h| h.ty() == e_ty).unwrap()
}

pub trait PacketHandler: Sync + Send {
    fn handle(&self, client: &mut Client, data: Value) -> Result<()>;

    fn ty(&self) -> EventType;
}