use std::{any::Any, mem};

pub mod client;

use crate::Client;

pub use self::{
    client::Ready
};

pub trait Event: 'static {
    fn ty() -> EventType where Self: Sized;

    fn as_any(&self) -> &dyn Any;
}

#[derive(PartialEq)]
pub enum EventType {
    Ready
}

impl EventType {
    pub fn from_str(id: &str) -> Option<Self> {
        use EventType::*;

        match id {
            "READY" => Some(Ready),
            _       => None
        }
    }
}

pub struct Listener {
    pub ty:     EventType,
    pub call:   fn(&dyn Event, fn(*const ()), &Client),
    pub i_call: fn(*const ())
}

impl Listener {
    pub fn new<E: Event>(ty: EventType, callback: fn(&E, &Client)) -> Self {
        Self {
            ty,
            call:   unsafe { mem::transmute(Self::handle::<E> as fn(_, _, _)) },
            i_call: unsafe { mem::transmute(callback) }
        }
    }

    fn handle<E: Event>(event: &dyn Event, call: fn(&E, &Client), c: &Client) {
        let event: &E = event.as_any().downcast_ref().unwrap();
        call(event, c);
    }
}

macro_rules! impl_event {
    ($t: ty, $et: path) => {
        impl crate::event::Event for $t {
            fn ty() -> EventType {
                $et
            }

            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
        }
    };
}

pub(crate) use impl_event;