use std::{marker::PhantomData, mem::MaybeUninit};

pub struct Stored<'scope, T: 'scope> {
    inner: T,
    _phantom: PhantomData<&'scope ()>,
}

impl<'scope, T: 'scope> Stored<'scope, T> {
    pub fn store<F>(mut me: MaybeUninit<Self>, f: F) -> Self
    where
        F: FnOnce(Store<'scope, T>) + 'scope,
    {
        let inner = unsafe { &mut *me.as_mut_ptr().cast() };
        f(Store { inner });
        unsafe { me.assume_init() }
    }
}

pub struct Store<'scope, T> {
    inner: &'scope mut MaybeUninit<T>,
}

impl<'scope, Rest: 'scope, T: 'scope> Store<'scope, (Rest, T)> {
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
