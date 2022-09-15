use serde::Deserialize;

use super::User;

#[derive(Deserialize, Debug)]
pub struct Team {
    #[serde(rename = "icon")]
    pub icon_hash: Option<String>,
    pub id:        String,
    pub members:   Vec<TeamMember>,
    pub name:      String,
    #[serde(rename = "owner_user_id	")]
    pub owner_id:  String	
}

#[derive(Deserialize, Debug)]
pub struct TeamMember {
    pub user: User
}

pub enum MembershipState {
    Invited,
    Accepted
}