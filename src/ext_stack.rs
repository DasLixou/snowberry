use std::mem::{ManuallyDrop, MaybeUninit};

use crate::recursive::Rec;

// #[repr(transparent)]
// pub struct ExtStack<T> {
//     inner: T,
// }

#[macro_export]
macro_rules! extend_for {
    ($f:expr) => {{
        use ::core::mem::MaybeUninit;
        use $crate::ext_stack::{ExtStackRef, Owned};

        let mut uninit: MaybeUninit<_> = MaybeUninit::uninit();

        fn cast_same_lifetime<'a, T>(from: &'a mut MaybeUninit<T>) -> &'a mut () {
            unsafe { &mut *from.as_mut_ptr().cast() }
        }

        let uninit_ref: &mut MaybeUninit<_> = &mut uninit;
        let uninit_ref: &mut () = cast_same_lifetime(uninit_ref);
        let ext_ref = ExtStackRef {
            // SAFETY: We can only run the drop of the inner type after calling `assume_init`, which we only do at the very end, after we defused the owned wrapper.
            inner: Owned(uninit_ref),
        };
        {
            fn call<'life, T>(closure: impl FnOnce(ExtStackRef<'life, ()>) -> ExtStackRef<'life, T>, ext_ref: ExtStackRef<'life, ()>) -> ExtStackRef<'life, T> {
                closure(ext_ref)
            }
            let ext_ref = call(($f), ext_ref);
            let _ = ext_ref.inner.defuse();
        }
        unsafe { uninit.assume_init() }
    }};
}

// impl<T> ExtStack<T> {
//     pub fn extend_for<'life, F>(f: F) -> Self
//     where
//         F: FnOnce(ExtStackRef<'life, ()>) -> ExtStackRef<'life, T>,
//         T: 'life,
//     {
//         let mut uninit: MaybeUninit<T> = MaybeUninit::uninit();

//         fn cast_same_lifetime<'a, T>(from: &'a mut MaybeUninit<T>) -> &'a mut () {
//             unsafe { &mut *from.as_mut_ptr().cast() }
//         }

//         let uninit_ref: &mut MaybeUninit<T> = &mut uninit;
//         let uninit_ref: &mut () = cast_same_lifetime(uninit_ref);
//         let ext_ref = ExtStackRef {
//             // SAFETY: We can only run the drop of the inner type after calling `assume_init`, which we only do at the very end, after we defused the owned wrapper.
//             inner: Owned(uninit_ref),
//         };
//         {
//             let ext_ref = f(ext_ref);
//             let _ = ext_ref.inner.defuse();
//         }
//         ExtStack {
//             inner: unsafe { uninit.assume_init() },
//         }
//     }
// }

#[repr(transparent)]
pub struct Owned<'life, T>(pub &'life mut T);
impl<'life, T> Owned<'life, T> {
    pub fn defuse(self) -> &'life mut T {
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

pub struct ExtStackRef<'life, Init> {
    pub inner: Owned<'life, Init>,
}

impl<'life, Init> ExtStackRef<'life, Init> {
    pub(crate) unsafe fn ptr_mut(&self) -> *mut () {
        self.inner.0 as *const _ as *mut ()
    }

    pub fn store<T>(self, val: T) -> (ExtStackRef<'life, Rec<Init, T>>, usize) {
        unsafe {
            let extended_inner = core::mem::transmute::<
                Owned<'_, Init>,
                Owned<'_, Rec<Init, MaybeUninit<T>>>,
            >(self.inner);
            let (inner, next_uninit) = extended_inner.defuse().split();
            let inner = Owned(inner);

            next_uninit.write(val);

            let inner_with_next =
                core::mem::transmute::<Owned<'life, Init>, Owned<'life, Rec<Init, T>>>(inner);
            (
                ExtStackRef {
                    inner: inner_with_next,
                },
                Rec::<Init, T>::offset(),
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{
        panic::catch_unwind,
        sync::atomic::{AtomicU8, Ordering},
    };

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
            let stack: () = extend_for!(|cx| {
                let (cx, _) = cx.store(Dropper);
                let (cx, _) = cx.store(Dropper);
                panic!();
                let (cx, _) = cx.store(Dropper);
                let (cx, _) = cx.store(Dropper);
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
