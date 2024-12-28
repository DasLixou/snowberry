use std::{mem::MaybeUninit, pin::pin};

use crate::{composable::Composable, composition::Composition};

#[macro_export]
macro_rules! run_simple {
    ($($tokens:tt)*) => {
        let composition =  ::std::pin::pin!(::std::mem::MaybeUninit::uninit());
        let composition = unsafe { ::snowberry::composition::Composition::open(composition, $($tokens)*) };
        unsafe {
            composition.get_unchecked_mut().assume_init_drop();
        }
    };
}

pub fn run_simple(c: impl for<'life> Composable<'life>) {
    let composition = pin!(MaybeUninit::uninit());
    let composition = unsafe { Composition::open(composition, c) };
    unsafe {
        composition.get_unchecked_mut().assume_init_drop();
    }
}
