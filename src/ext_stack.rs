use std::{marker::PhantomData, mem::MaybeUninit};

use crate::{generics_stack::RecursiveTuple, responsible_pin::ResponsiblePin, uninit::Uninit};

#[repr(transparent)]
pub struct ExtStack<T> {
    inner: T,
}

impl<T> ExtStack<T> {
    pub fn extend_for<'life, F>(
        me: ResponsiblePin<'life, Uninit<Self>>,
        f: F,
    ) -> ResponsiblePin<'life, Self>
    where
        F: FnOnce(ExtStackRef<'life, (), T>) -> ExtStackRef<'life, T, ()>,
    {
        unsafe {
            let inner = me.map_uninit(|me| &raw mut (*me).inner);
            let with_init = inner.map(|inner: &mut Uninit<T>| {
                let ptr = inner as *mut _;
                let ptr = ptr as *mut ();
                &mut *ptr
            });
            let ext_ref = ExtStackRef {
                inner: with_init,
                todo: PhantomData::<T>,
            };
            let ext_ref = f(ext_ref);
            let inner = ext_ref.inner;
            inner.map(|mut inner| &mut *(&raw mut inner).cast::<ExtStack<_>>())
        }
    }
}

pub struct ExtStackRef<'life, Init, Todo> {
    inner: ResponsiblePin<'life, Init>,
    todo: PhantomData<Todo>,
}

impl<'life, Init, Todo: RecursiveTuple> ExtStackRef<'life, Init, Todo> {
    pub fn store(self, val: Todo::Pop) -> ExtStackRef<'life, (Init, Todo::Pop), Todo::Remainder> {
        self.store_with(|mu| mu.write(val))
    }

    pub fn store_with<F>(self, f: F) -> ExtStackRef<'life, (Init, Todo::Pop), Todo::Remainder>
    where
        F: FnOnce(&mut MaybeUninit<Todo::Pop>) -> &mut Todo::Pop,
    {
        unsafe {
            let inner_with_next_uninit = core::mem::transmute::<
                ResponsiblePin<'life, Init>,
                ResponsiblePin<'life, (Init, Uninit<Todo::Pop>)>,
            >(self.inner);
            let (inner, next_uninit) = inner_with_next_uninit.split();
            // TODO: clean up
            let _ = next_uninit.map_uninit(|next: *mut <Todo as RecursiveTuple>::Pop| {
                let mu = &mut *next.cast::<MaybeUninit<_>>();
                let _ = f(mu);
                next
            });
            let inner_with_next = core::mem::transmute::<
                ResponsiblePin<'life, Init>,
                ResponsiblePin<'life, (Init, Todo::Pop)>,
            >(inner);
            ExtStackRef {
                inner: inner_with_next,
                todo: PhantomData,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{
        panic::catch_unwind,
        sync::atomic::{AtomicU8, Ordering},
    };

    use crate::{ext_stack::ExtStack, responsible_pin, uninit::Uninit};

    #[test]
    fn assert_correct_drop() {
        static COUNT: AtomicU8 = AtomicU8::new(0);
        struct Dropper;
        impl Drop for Dropper {
            fn drop(&mut self) {
                COUNT.fetch_add(1, Ordering::Relaxed);
            }
        }
        let res = catch_unwind(|| {
            responsible_pin!(let unsafe stack = Uninit::uninit());
            let stack = ExtStack::extend_for(stack, |cx| {
                let cx = cx.store(Dropper);
                let cx = cx.store(Dropper);
                let cx = cx.store_with(|a| {
                    a.write(Dropper); // this won't be dropped
                    panic!();
                });
                let cx = cx.store(Dropper);
                cx
            });
            drop(stack);
        });
        assert!(res.is_err());
        assert_eq!(COUNT.load(Ordering::Relaxed), 2);
    }
}
