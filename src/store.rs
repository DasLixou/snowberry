use std::{marker::PhantomData, mem::MaybeUninit, pin::Pin};

use crate::{composable::Composable, composition::Composition};

pub struct Stored<'scope, T: 'scope> {
    inner: T,
    _phantom: PhantomData<&'scope ()>,
}

impl<'scope, T: 'scope> Stored<'scope, T> {
    pub fn store<C>(me: Pin<&'scope mut MaybeUninit<Self>>, c: C)
    where
        C: Composable<'scope, Store = T>,
    {
        let inner = unsafe {
            me.map_unchecked_mut(|unpin_me| &mut *unpin_me.as_mut_ptr().cast::<MaybeUninit<_>>())
        };
        c.compose(Store { inner });
    }
}

pub struct Store<'scope, T> {
    inner: Pin<&'scope mut MaybeUninit<T>>,
}

impl<'scope, Rest: 'scope, T: 'scope> Store<'scope, (Rest, T)> {
    #[must_use]
    pub fn split_off(self) -> (Store<'scope, Rest>, Pin<&'scope mut MaybeUninit<T>>) {
        unsafe {
            let unpin = self.inner.get_unchecked_mut();
            let inner = Pin::new_unchecked(&mut *(&raw mut (*unpin.as_mut_ptr()).0).cast());
            let slot = Pin::new_unchecked(&mut *(&raw mut (*unpin.as_mut_ptr()).1).cast());
            (Store { inner }, slot)
        }
    }

    #[must_use]
    pub fn store_val(self, val: T) -> (Store<'scope, Rest>, Pin<&'scope mut T>) {
        let (store, slot) = self.split_off();
        let slot = unsafe { slot.map_unchecked_mut(|unpin| unpin.write(val)) };
        (store, slot)
    }
}

impl<'scope, Rest: 'scope, C> Store<'scope, (Rest, Composition<'scope, C>)>
where
    C: Composable<'scope>,
{
    #[must_use]
    pub fn compose(self, c: C) -> Store<'scope, Rest> {
        let (store, mem) = self.split_off();
        unsafe {
            // SAFETY: drops because it is stored as `Composition<'scope, C>` in the tuple.
            Composition::open(mem, c);
        }
        store
    }
}

impl Store<'_, ()> {
    pub fn end(self) {
        let _ = self;
    }
}
