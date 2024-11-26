use std::any::{Any, TypeId};

use crate::application::App;

pub trait Event: Any {}

pub struct EventDispatcher<'a> {
    event: &'a dyn Event,
}

impl<'a> EventDispatcher<'a> {
    pub fn new(event: &'a impl Event) -> Self {
        Self { event }
    }

    pub fn dispatch<E: Event>(&self, app: &mut App, app_fn: impl Fn(&mut App)) {
        if self.event.type_id() == TypeId::of::<E>() {
            app_fn(app);
        }
    }
}
