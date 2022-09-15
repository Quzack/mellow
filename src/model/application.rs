use serde::Deserialize;

use super::User;

#[derive(Deserialize, Debug)]
pub struct Application {
    pub id:                     String,
    pub name:                   String,
    #[serde(rename = "icon")]
    pub icon_hash:              Option<String>,
    pub description:            String,
    pub bot_public:             bool,
    pub bot_require_code_grant: bool,
    #[serde(rename = "terms_of_service_url")]
    pub tos_url:                Option<String>,
    #[serde(rename = "privacy_policy_url")]
    pub priv_pol_url:           Option<String>,
    pub owner:                  Option<User>,
    pub verify_key:             String,
    
}