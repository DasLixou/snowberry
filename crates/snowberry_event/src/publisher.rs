use std::ops::AddAssign;

use crate::Event;

type BoxedSubscriber<E> = Box<dyn Fn(E)>;

pub struct Publisher<E: Event> {
    subscribers: Vec<BoxedSubscriber<E>>,
}

impl<E: Event> Publisher<E> {
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

impl<E: Event, F: Fn(E) + 'static> AddAssign<F> for Publisher<E> {
    fn add_assign(&mut self, rhs: F) {
        self.subscribers.push(Box::new(rhs));
    }
}
