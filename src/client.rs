use crate::{gateway::DiscordWsClient, Result, event::{Event, Listener}};

pub struct Client<'a> {
    pub token: &'a str,
    pub intents: &'a u16,
    listeners: Vec<Listener>
}

impl<'a> Client<'a> {
    pub fn new(token: &'a str, intents: &'a u16) -> Self {
        Self {
            token,
            intents,
            listeners: vec![]
        }
    }

    pub fn from_token(token: &'a str) -> Self {
        Self::new(token, &0)
    }

    pub fn on_event<E: Event>(&mut self, f: fn(&E)) {
        self.listeners.push(Listener::new::<E>(E::ty(), f));
    }

    pub fn emit_event<E: Event>(&self, inst: E) {
        for listener in self.listeners.iter() {
            if listener.ty == E::ty() {
                (listener.call)(&inst, listener.i_call);
            }
        }
    }

    pub async fn start(self) -> Result<()> {
        DiscordWsClient { client: self }.open_connection().await
    }
}