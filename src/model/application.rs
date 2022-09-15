use serde::Deserialize;

use super::{User, Team};

#[derive(Deserialize, Debug)]
pub struct Application {
    pub id:                     String,
    pub name:                   Option<String>,
    #[serde(rename = "icon")]
    pub icon_hash:              Option<String>,
    pub description:            Option<String>,
    pub bot_public:             Option<bool>,
    pub bot_require_code_grant: Option<bool>,
    #[serde(rename = "terms_of_service_url")]
    pub tos_url:                Option<String>,
    #[serde(rename = "privacy_policy_url")]
    pub priv_pol_url:           Option<String>,
    pub owner:                  Option<User>,
    pub verify_key:             Option<String>,
    pub team:                   Option<Team>,
    pub guild_id:               Option<String>,
    pub slug:                   Option<String>,
    #[serde(rename = "cover_image")]
    pub cover_hash:             Option<String>,
    pub flags:                  Option<usize>,
    pub tags:                   Option<Vec<String>>,
    // pub install_params:      ??
    #[serde(rename = "custom_install_url")]
    pub install_url:            Option<String>     
}