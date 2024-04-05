use std::{any::Any, cell::RefCell, marker::PhantomData};

use slotmap::new_key_type;

new_key_type! {
    pub struct ScopeKey;
}

#[derive(Clone, Copy)]
pub struct ScopeLife<'scope>(pub PhantomData<&'scope ()>);

pub struct Scope {
    store: RefCell<Vec<Box<dyn Any>>>,
    pub sub_scopes: Vec<ScopeKey>,
}

impl Scope {
    pub fn new() -> Self {
        Self {
            store: RefCell::new(vec![]),
            sub_scopes: vec![],
        }
    }

    pub fn store<'scope, T: 'static>(&self, _life: ScopeLife<'scope>, val: T) -> &'scope T {
        let mut store = self.store.borrow_mut();
        let len = store.len();
        store.push(Box::new(val));
        drop(store);
        let store = self.store.borrow();
        let b = store.get(len).unwrap().downcast_ref::<T>().unwrap();
        unsafe {
            // TODO: think about safety
            core::mem::transmute(b)
        }
    }
}
