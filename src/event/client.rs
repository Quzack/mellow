use crate::{event::{impl_event, EventType}, model::guild::UnavailableGuild};

pub struct Ready {
    pub guilds: Vec<UnavailableGuild>
}

impl_event!(Ready, EventType::Ready);