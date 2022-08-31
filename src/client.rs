use crate::gateway::DiscordWsClient;

pub struct Client<'a> {
    pub token: &'a str,
    intents: &'a u16
}

impl<'a> Client<'a> {
    pub fn new(token: &'a str, intents: &'a u16) -> Self {
        Self {
            token,
            intents
        }
    }

    pub fn from_token(token: &'a str) -> Self {
        Self::new(token, &0)
    }

    pub async fn start(&self) -> crate::Result<()> {
        DiscordWsClient { token: self.token, intents: self.intents }.open_connection().await
    }
}