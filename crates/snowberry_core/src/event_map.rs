use std::{
    any::{Any, TypeId},
    marker::PhantomData,
};

use slotmap::SlotMap;

use crate::event::{Event, EventIdx};

pub struct EventMap {
    inner: SlotMap<EventIdx, Vec<Box<dyn ErasedSubscriber>>>,
}

impl EventMap {
    pub fn new() -> Self {
        Self {
            inner: SlotMap::with_key(),
        }
    }

    pub fn insert<T>(&mut self) -> Event<T> {
        Event::from(self.inner.insert(Vec::new()))
    }

    pub fn push_sub<T: 'static, S>(&mut self, event: Event<T>, subscriber: S)
    where
        S: Fn(&T) + 'static, // TODO: we dont want 'static
    {
        self.inner[event.idx].push(Box::new(Subscriber {
            sub: subscriber,
            phantom: PhantomData,
        }))
    }
}

pub struct Subscriber<P, S: Fn(&P)> {
    phantom: PhantomData<P>,
    sub: S,
}

pub trait ErasedSubscriber {
    // SAFETY: type of val must be the type of the param of the subscriber
    unsafe fn call(&self, val: &dyn Any);
}

impl<P: 'static, S: Fn(&P)> ErasedSubscriber for Subscriber<P, S> {
    unsafe fn call(&self, val: &dyn Any) {
        debug_assert_eq!(val.type_id(), TypeId::of::<P>());
        (self.sub)(val.downcast_ref().unwrap_unchecked())
    }
}
