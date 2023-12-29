use snowberry_arena::DropArena;
use type_map::TypeMap;

use crate::{
    event::{Event, Listener},
    event_map::EventMap,
};

pub struct Context<'g, 's> {
    pub global_resources: &'g mut TypeMap,
    pub(crate) bank: &'s DropArena,
    pub(crate) cleanup: &'s mut Vec<Box<dyn FnOnce(&mut EventMap)>>,
    pub(crate) events: &'g mut EventMap,
}

impl<'g, 's> Context<'g, 's> {
    pub fn deposit<T>(&self, val: T) -> &'s T {
        self.bank.alloc(val)
    }

    pub fn event<T>(&mut self) -> Event<T> {
        Event::from(self.events.insert_event())
    }

    pub fn listener<T: 'static, L>(&mut self, listener: L) -> Listener<T>
    where
        L: Fn(&T) + 's,
    {
        // SAFETY: we insert cleanup function
        let l = unsafe { self.events.insert_listener(listener) };
        self.cleanup.push(Box::new(move |events| {
            events.remove_listener(l);
        }));
        l
    }

    pub fn subscribe<T>(&mut self, event: Event<T>, listener: Listener<T>) {
        self.events.push_sub(event, listener);
    }

    pub fn publish<T: 'static>(&mut self, event: Event<T>, data: T) {
        self.events.publish(event, data);
    }
}
