use std::mem::MaybeUninit;

use crate::{composable::Composable, scope::Scope};

pub struct Composition<'scope, C: Composable<'scope>> {
    scope: Scope<'scope>,
    stored: C::Store,
}

impl<'scope, C: Composable<'scope>> Composition<'scope, C> {
    pub fn open(mut me: MaybeUninit<Self>, c: C) {
        let store = unsafe { &mut (*me.as_mut_ptr()).stored };
        let (_scope, _) = Scope::open(|_scope| c.compose(store));
    }
}
