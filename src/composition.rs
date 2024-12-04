use std::mem::MaybeUninit;

use crate::{composable::Composable, scope::Scope, store::Stored};

pub struct Composition<'scope, C: Composable<'scope>> {
    scope: Scope<'scope>,
    stored: Stored<'scope, C::Store>,
}

impl<'scope, C: Composable<'scope>> Composition<'scope, C> {
    pub fn open(c: C) -> Self {
        let stored = MaybeUninit::uninit();
        let (scope, stored) = Scope::open(|_scope| Stored::store(stored, c));
        Composition { scope, stored }
    }
}
