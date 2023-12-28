use std::ops::AddAssign;

use crate::Event;

type BoxedSubscriber<'s, E> = Box<dyn Fn(E) + 's>;

pub struct Publisher<'s, E: Event> {
    subscribers: Vec<BoxedSubscriber<'s, E>>,
}

impl<'s, E: Event> Publisher<'s, E> {
    pub fn new() -> Self {
        Self {
            subscribers: Vec::new(),
        }
    }

    pub fn publish(&self, event: E) {
        for sub in &self.subscribers {
            (sub)(event.clone());
        }
    }
}

impl<'s, E: Event, F: Fn(E) + 's> AddAssign<F> for Publisher<'s, E> {
    fn add_assign(&mut self, rhs: F) {
        self.subscribers.push(Box::new(rhs));
    }
}
