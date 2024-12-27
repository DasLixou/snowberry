use std::{mem::MaybeUninit, pin::Pin};

use crate::{composable::Composable, scope::Scope, store::Stored};

pub struct Composition<'scope, C: Composable<'scope>> {
    scope: Scope<'scope>,
    stored: Stored<'scope, C::Store>,
}

impl<'scope, C: Composable<'scope>> Composition<'scope, C> {
    /// ## Safety
    /// * Caller has to ensure that `me` gets dropped as an initialized composition.
    pub unsafe fn open(
        me: Pin<&'scope mut MaybeUninit<Self>>,
        c: C,
    ) -> Pin<&'scope mut MaybeUninit<Self>> {
        let unpin_me = unsafe { me.get_unchecked_mut() };
        let stored = unsafe {
            Pin::new_unchecked(
                &mut *(&raw mut (*unpin_me.as_mut_ptr()).stored).cast::<MaybeUninit<_>>(),
            )
        };
        let (scope, _) = Scope::open(|_scope| Stored::store(stored, c));
        unsafe {
            (&raw mut (*unpin_me.as_mut_ptr()).scope).write(scope);
            Pin::new_unchecked(unpin_me)
        }
    }
}
