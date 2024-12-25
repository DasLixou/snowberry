use std::{mem::MaybeUninit, pin::Pin};

use crate::{composable::Composable, scope::Scope, store::Stored};

pub struct Composition<'scope, C: Composable<'scope>> {
    scope: Scope<'scope>,
    stored: Stored<'scope, C::Store>,
}

impl<'scope, C: Composable<'scope>> Composition<'scope, C> {
    pub fn open(me: Pin<&'scope mut MaybeUninit<Self>>, c: C) {
        let stored = unsafe {
            me.map_unchecked_mut(|unpin_me| {
                &mut *(&raw mut (*unpin_me.as_mut_ptr()).stored).cast::<MaybeUninit<_>>()
            })
        };
        let (_scope, _) = Scope::open(|_scope| Stored::store(stored, c));
    }
}
