use std::any::Any;

use slotmap::{new_key_type, SlotMap};

new_key_type! { pub struct ConstructId; }

pub struct Constructs {
    constructs: SlotMap<ConstructId, Box<dyn Any>>,
}

impl Constructs {
    pub fn new() -> Self {
        Self {
            constructs: SlotMap::with_key(),
        }
    }

    pub fn insert(&mut self, b: Box<dyn Any>) -> ConstructId {
        self.constructs.insert(b)
    }
}
