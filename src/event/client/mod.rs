use crate::event::{impl_event, EventType};

pub struct Ready;

impl_event!(Ready, EventType::Ready);