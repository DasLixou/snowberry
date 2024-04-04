use std::any::Any;

use slotmap::new_key_type;

new_key_type! {
    pub struct ScopeKey;
}

pub struct Scope {
    store: Vec<Box<dyn Any>>,
    pub sub_scopes: Vec<ScopeKey>,
}

impl Scope {
    pub fn new() -> Self {
        Self {
            store: vec![],
            sub_scopes: vec![],
        }
    }

    pub fn store<T: 'static>(&mut self, val: T) {
        self.store.push(Box::new(val));
    }
}
