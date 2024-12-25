use std::mem::MaybeUninit;

use crate::{composable::Composable, scope::Scope, store::Stored};

pub struct Composition<'scope, C: Composable<'scope>> {
    scope: Scope<'scope>,
    stored: Stored<'scope, C::Store>,
}

impl<'scope, C: Composable<'scope>> Composition<'scope, C> {
    pub fn open(mut me: &mut MaybeUninit<Self>, c: C) {
        let stored = get_stored(me);
        let (_scope, _) = Scope::open(|_scope| Stored::store(stored, c));
    }
}

fn get_stored<'a, 'scope, C: Composable<'scope>>(
    comp: &'a mut MaybeUninit<Composition<'scope, C>>,
) -> &'a mut MaybeUninit<Stored<'scope, C::Store>> {
    let ptr = unsafe {
        let ptr = comp.as_mut_ptr();
        &mut (*ptr).stored as *mut _ as *mut MaybeUninit<_>
    };
    unsafe { &mut *ptr }
}
