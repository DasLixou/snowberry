use std::{
    any::{Any, TypeId},
    marker::PhantomData,
};

use slotmap::SlotMap;

use crate::event::{Event, EventIdx, Listener, ListenerIdx};

pub struct EventMap {
    events: SlotMap<EventIdx, Vec<ListenerIdx>>,
    listeners: SlotMap<ListenerIdx, Box<dyn ErasedSubscriber>>,
}

impl EventMap {
    pub fn new() -> Self {
        Self {
            events: SlotMap::with_key(),
            listeners: SlotMap::with_key(),
        }
    }

    pub fn insert_event<T>(&mut self) -> Event<T> {
        Event::from(self.events.insert(Vec::new()))
    }

    /// SAFETY: Make sure that the Fn behind S is removed as a listener when it's lifetime scope is dead
    pub unsafe fn insert_listener<T, S>(&mut self, listener: S) -> Listener<T>
    where
        T: 'static,
        S: Fn(&T),
    {
        let container: Box<dyn ErasedSubscriber> = Box::new(SubContainer {
            sub: listener,
            phantom: PhantomData,
        });
        let container: Box<dyn ErasedSubscriber + 'static> = core::mem::transmute(container);
        Listener::from(self.listeners.insert(container))
    }

    pub fn remove_listener<T>(&mut self, listener: Listener<T>) {
        self.listeners.remove(listener.idx);
    }

    pub fn push_sub<T>(&mut self, event: Event<T>, listener: Listener<T>) {
        self.events[event.idx].push(listener.idx)
    }

    pub fn publish<T: 'static>(&mut self, event: Event<T>, data: T) {
        self.events[event.idx].retain(|l| {
            if let Some(listener) = self.listeners.get(*l) {
                unsafe {
                    // SAFETY: we only insert matching subs
                    listener.call(&data);
                }
                true
            } else {
                false
            }
        })
    }
}

pub struct SubContainer<P, S: Fn(&P)> {
    phantom: PhantomData<P>,
    sub: S,
}

pub trait ErasedSubscriber {
    // SAFETY: type of val must be the type of the param of the subscriber
    unsafe fn call(&self, val: &dyn Any);
}

impl<P: 'static, S: Fn(&P)> ErasedSubscriber for SubContainer<P, S> {
    unsafe fn call(&self, val: &dyn Any) {
        debug_assert_eq!(val.type_id(), TypeId::of::<P>());
        (self.sub)(val.downcast_ref().unwrap_unchecked())
    }
}
