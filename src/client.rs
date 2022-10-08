use crate::{gateway::WsClient, Result, event::{Event, Listener}, model::{User, Application}};

pub struct Client<'a> {
    pub user:  Option<User>,
    pub app:   Option<Application>,
    token:     &'a str,
    intents:   u16,
    listeners: Vec<Listener>
}

impl<'a> Client<'a> {
    pub fn new(token: &'a str, intents: u16) -> Self {
        Self {
            user:      None,
            app:       None,
            token,
            intents,
            listeners: vec![]
        }
    }

    pub fn from_token(token: &'a str) -> Self {
        Self::new(token, 0)
    }

    pub fn on_event<E: Event>(&mut self, f: fn(&E, &Client)) {
        self.listeners.push(Listener::new::<E>(E::ty(), f));
    }

    pub fn emit_event<E: Event>(&self, inst: E) {
        self.listeners.iter().filter(|l| l.ty == E::ty()).for_each(|l| (l.call)(&inst, l.i_call, self));
    }

    pub fn get_token(&self) -> &'a str {
        self.token
    }

    pub fn get_intents(&self) -> &u16 {
        &self.intents
    }

    pub async fn start(self) -> Result<()> {
        WsClient::new(self).connect().await
    }
}