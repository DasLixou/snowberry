use std::{marker::PhantomData, ops::AddAssign};

use crate::Event;

pub struct Publisher<E: Event> {
    phantom: PhantomData<E>,
}

impl<E: Event> Publisher<E> {
    pub fn new() -> Self {
        Self {
            phantom: PhantomData,
        }
    }

    pub fn publish(&self, _event: E) {}
}

impl<E: Event> AddAssign<()> for Publisher<E> {
    fn add_assign(&mut self, _rhs: ()) {}
}
