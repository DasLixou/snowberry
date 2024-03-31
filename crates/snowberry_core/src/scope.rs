use std::any::Any;

pub struct Scope {
    store: Vec<Box<dyn Any>>,
}

impl Scope {
    pub fn new() -> Self {
        Self { store: vec![] }
    }

    pub fn store<T: 'static>(&mut self, val: T) {
        self.store.push(Box::new(val));
    }
}
