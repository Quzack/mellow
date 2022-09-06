#[macro_use]

pub mod client;

pub use self::{
    client::*
};

pub trait Event {
    fn call_type() -> EventType;
}

pub enum EventType {
    Ready
}

macro_rules! impl_event {
    ($t: ty, $et: path) => {
        impl crate::event::Event for $t {
            fn call_type() -> EventType {
                $et
            }
        }
    };
}

pub(crate) use impl_event;