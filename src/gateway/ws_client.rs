const DISCORD_GATEWAY_URL: &str = "wss://gateway.discord.gg/?v=10&encoding=json";

pub struct DiscordWsClient<'a> {
    pub token: &'a str,
    pub intents: &'a u16
}

impl<'a> DiscordWsClient<'a> {
    pub async fn open_connection(&self) -> crate::Result<()> {
        Ok(())
    }
}