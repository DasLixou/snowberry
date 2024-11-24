use std::mem::MaybeUninit;

use crate::{
    scope::Scope,
    store::{Store, Stored},
};

pub struct Composition<'scope, S> {
    scope: Scope<'scope>,
    stored: Stored<'scope, S>,
}

impl<'scope, S> Composition<'scope, S> {
    pub fn open<F>(f: F) -> Self
    where
        F: FnOnce(Store<'scope, S>) + 'scope,
    {
        let stored = MaybeUninit::uninit();
        let (scope, stored) = Scope::open(|_scope| Stored::store(stored, f));
        Composition { scope, stored }
    }
}
