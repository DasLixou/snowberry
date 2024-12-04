use std::{marker::PhantomData, mem::MaybeUninit};

use crate::composable::Composable;

pub struct Stored<'scope, T: 'scope> {
    inner: T,
    _phantom: PhantomData<&'scope ()>,
}

pub struct Store<'scope, T> {
    inner: &'scope mut MaybeUninit<T>,
}

impl<'scope, Rest: 'scope, T: 'scope> Store<'scope, (Rest, T)> {
    #[must_use]
    pub fn split_off(self) -> (Store<'scope, Rest>, &'scope mut MaybeUninit<T>) {
        unsafe {
            (
                Store {
                    inner: &mut *(&raw mut (*self.inner.as_mut_ptr()).0).cast(),
                },
                &mut *(&raw mut (*self.inner.as_mut_ptr()).1).cast(),
            )
        }
    }

    #[must_use]
    pub fn store_val(self, val: T) -> (Store<'scope, Rest>, &'scope mut T) {
        let (store, slot) = self.split_off();
        (store, slot.write(val))
    }
}

impl Store<'_, ()> {
    pub fn end(self) {
        let _ = self;
    }
}
