use std::marker::PhantomData;

use bumpalo::Bump;
use slotmap::new_key_type;

new_key_type! {
    pub struct ScopeKey;
}

#[derive(Clone, Copy)]
pub struct ScopeLife<'scope>(pub PhantomData<&'scope ()>);

pub struct Scope {
    store: Bump,
    pub sub_scopes: Vec<ScopeKey>,
}

impl Scope {
    pub fn new() -> Self {
        Self {
            store: Bump::new(),
            sub_scopes: vec![],
        }
    }

    pub fn store<'scope, T: 'scope>(&self, _life: ScopeLife<'scope>, val: T) -> &'scope T {
        let b = self.store.alloc(val);
        unsafe {
            // TODO: think about safety
            core::mem::transmute(b)
        }
    }
}
