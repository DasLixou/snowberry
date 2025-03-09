use std::{
    marker::PhantomData,
    mem::{ManuallyDrop, MaybeUninit},
};

use crate::recursive::{Rec, Recursive};

#[repr(transparent)]
pub struct ExtStack<T> {
    inner: T,
}

impl<T> ExtStack<T> {
    pub fn extend_for<F>(f: F) -> Self
    where
        for<'life> F: FnOnce(ExtStackRef<'life, (), T>) -> ExtStackRef<'life, T, ()>,
    {
        let mut uninit: MaybeUninit<T> = MaybeUninit::uninit();

        fn cast_same_lifetime<'a, T>(from: &'a mut MaybeUninit<T>) -> &'a mut () {
            unsafe { &mut *from.as_mut_ptr().cast() }
        }

        let uninit_ref: &mut MaybeUninit<T> = &mut uninit;
        let uninit_ref: &mut () = cast_same_lifetime(uninit_ref);
        let ext_ref = ExtStackRef {
            // SAFETY: We can only run the drop of the inner type after calling `assume_init`, which we only do at the very end, after we defused the owned wrapper.
            inner: Owned(uninit_ref),
            todo: PhantomData::<T>,
        };
        {
            let ext_ref = f(ext_ref);
            let _ = ext_ref.inner.defuse();
        }
        ExtStack {
            inner: unsafe { uninit.assume_init() },
        }
    }
}

#[repr(transparent)]
struct Owned<'life, T>(&'life mut T);
impl<'life, T> Owned<'life, T> {
    fn defuse(self) -> &'life mut T {
        let this = ManuallyDrop::new(self);
        unsafe { core::ptr::read(&this.0) }
    }
}
impl<'life, T> Drop for Owned<'life, T> {
    fn drop(&mut self) {
        unsafe {
            core::ptr::drop_in_place(self.0);
        }
    }
}

pub struct ExtStackRef<'life, Init, Todo> {
    inner: Owned<'life, Init>,
    todo: PhantomData<Todo>,
}

impl<'life, Init, Todo: Recursive> ExtStackRef<'life, Init, Todo> {
    pub fn store(
        self,
        val: Todo::Pop,
    ) -> ExtStackRef<'life, Rec<Init, Todo::Pop>, Todo::Remainder> {
        unsafe {
            let extended_inner = core::mem::transmute::<
                Owned<'_, Init>,
                Owned<'_, Rec<Init, MaybeUninit<Todo::Pop>>>,
            >(self.inner);
            let (inner, next_uninit) = extended_inner.defuse().split();
            let inner = Owned(inner);

            next_uninit.write(val);

            let inner_with_next = core::mem::transmute::<
                Owned<'life, Init>,
                Owned<'life, Rec<Init, Todo::Pop>>,
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

    use crate::ext_stack::ExtStack;

    #[test]
    fn correct_drop_amount() {
        static COUNT: AtomicU8 = AtomicU8::new(0);
        struct Dropper;
        impl Drop for Dropper {
            fn drop(&mut self) {
                COUNT.fetch_add(1, Ordering::Relaxed);
            }
        }
        let res = catch_unwind(|| {
            #[allow(unused_variables, unreachable_code)]
            let stack = ExtStack::extend_for(|cx| {
                let cx = cx.store(Dropper);
                let cx = cx.store(Dropper);
                panic!();
                let cx = cx.store(Dropper);
                let cx = cx.store(Dropper);
                cx
            });
            drop(stack);
        });
        assert!(res.is_err());
        assert_eq!(COUNT.load(Ordering::Relaxed), 2);
    }

    #[test]
    fn correct_drop_order() {
        todo!()
    }
}
