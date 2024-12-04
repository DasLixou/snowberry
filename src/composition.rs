use std::mem::MaybeUninit;

use crate::{composable::Composable, scope::Scope, store::Stored};

pub struct Composition<'scope, S> {
    scope: Scope<'scope>,
    stored: Stored<'scope, S>,
}

impl<'scope, S> Composition<'scope, S> {
    pub fn open<C>(c: C) -> Self
    where
        C: Composable<'scope, S>,
    {
        let stored = MaybeUninit::uninit();
        let (scope, stored) = Scope::open(|_scope| Stored::store(stored, c));
        Composition { scope, stored }
    }
}
