pub struct Client<'a> {
    pub token: &'a str,
    pub intents: &'a u16
}

impl<'a> Client<'a> {
    pub fn new(token: &'a str, intents: &'a u16) -> Self {
        Self {
            token,
            intents
        }
    }

    pub fn from_token(token: &'a str) -> Self {
        Self {
            token, 
            intents: &0
        }
    }

    pub async fn start() -> Result<(), crate::Error> {
        // TODO: Implementation.
        Ok(())
    }
}