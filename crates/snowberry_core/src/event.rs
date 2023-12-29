use std::marker::PhantomData;

use slotmap::new_key_type;

new_key_type! {
    pub struct EventIdx;
}

pub struct Event<T> {
    pub(crate) idx: EventIdx,
    phantom: PhantomData<T>,
}

impl<T> From<EventIdx> for Event<T> {
    fn from(idx: EventIdx) -> Self {
        Self {
            idx,
            phantom: PhantomData,
        }
    }
}

impl<T> Clone for Event<T> {
    fn clone(&self) -> Self {
        Event {
            idx: self.idx,
            phantom: PhantomData,
        }
    }
}
impl<T> Copy for Event<T> {}

new_key_type! {
    pub struct ListenerIdx;
}

pub struct Listener<T> {
    pub(crate) idx: ListenerIdx,
    phantom: PhantomData<T>,
}

impl<T> From<ListenerIdx> for Listener<T> {
    fn from(idx: ListenerIdx) -> Self {
        Self {
            idx,
            phantom: PhantomData,
        }
    }
}

impl<T> Clone for Listener<T> {
    fn clone(&self) -> Self {
        Listener {
            idx: self.idx,
            phantom: PhantomData,
        }
    }
}
impl<T> Copy for Listener<T> {}
