use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct UnavailableGuild {
    pub id: String
}