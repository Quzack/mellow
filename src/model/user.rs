use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct User {
    pub id:            String,
    #[serde(rename = "username")]
    pub name:          String,
    pub discriminator: String,
    #[serde(rename = "avatar")]
    pub avatar_hash:   Option<String>,
    pub bot:           Option<bool>,
    pub system:        Option<bool>,
    pub mfa_enabled:   Option<bool>,
    #[serde(rename = "banner")]
    pub banner_hash:   Option<String>,
    pub accent_color:  Option<u32>,
    pub locale:        Option<String>,
    pub verified:      Option<bool>,
    pub email:         Option<String>,
    pub flags:         Option<usize>,
    pub premium_type:  Option<u8>,
    pub public_flags:  Option<usize>
}