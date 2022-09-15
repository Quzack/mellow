use serde::Deserialize;
use serde_repr::Deserialize_repr;

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
    pub membership_state: MembershipState,
    pub permissions:      Vec<String>,
    pub team_id:          String,
    pub user:             User
}

#[derive(Deserialize_repr, Debug)]
#[repr(u8)]
pub enum MembershipState {
    Invited  = 1,
    Accepted = 2
}