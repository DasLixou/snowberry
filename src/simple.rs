use std::{mem::MaybeUninit, pin::pin, ptr::drop_in_place};

use crate::{composable::Composable, composition::Composition};

#[macro_export]
macro_rules! snowberry {
    ($($tokens:tt)*) => {
        let composition = pin!(::std::mem::MaybeUninit::uninit());
        let composition = unsafe { ::snowberry::composition::Composition::open(composition, $($tokens)*) };
        unsafe {
            ::std::ptr::drop_in_place(composition.get_unchecked_mut());
        }
    };
}

pub fn run_simple<F, C>(c: F)
where
    F: FnOnce() -> C,
    for<'a> C: Composable<'a>,
{
    let composition = pin!(MaybeUninit::uninit());
    let composition = unsafe { Composition::open(composition, c()) };
    unsafe {
        drop_in_place(composition.get_unchecked_mut());
    }
}
