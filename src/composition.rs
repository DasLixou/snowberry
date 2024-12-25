use std::mem::MaybeUninit;

use crate::{composable::Composable, scope::Scope, store::Stored};

pub struct Composition<'scope, C: Composable<'scope>> {
    scope: Scope<'scope>,
    stored: Stored<'scope, C::Store>,
}

impl<'scope, C: Composable<'scope>> Composition<'scope, C> {
    pub fn open(me: &mut MaybeUninit<Self>, c: C) {
        let stored = unsafe { &mut *(&raw mut (*me.as_mut_ptr()).stored).cast::<MaybeUninit<_>>() };
        let (_scope, _) = Scope::open(|_scope| Stored::store(stored, c));
    }
}
