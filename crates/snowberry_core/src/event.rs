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
