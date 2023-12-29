use bumpalo::Bump;
use type_map::TypeMap;

use crate::{event::Event, event_map::EventMap};

pub struct Context<'g, 's> {
    pub global_resources: &'g mut TypeMap,
    pub(crate) bank: &'s Bump,
    pub(crate) events: &'g mut EventMap,
}

impl<'g, 's> Context<'g, 's> {
    pub fn deposit<T>(&self, val: T) -> &'s T {
        self.bank.alloc(val)
    }

    pub fn event<T>(&mut self) -> Event<T> {
        Event::from(self.events.insert())
    }

    pub fn subscribe<T, S>(&self, event: Event<T>, subscriber: S)
    where
        S: Fn(&T), /* + 's */
    {
        todo!()
    }

    pub fn publish<T>(&self, event: Event<T>, data: T) {
        todo!()
    }
}
