use std::{mem::MaybeUninit, pin::pin, ptr::drop_in_place};

use crate::{composable::Composable, composition::Composition};

pub fn run_simple(c: impl for<'life> Composable<'life>) {
    let composition = pin!(MaybeUninit::uninit());
    let composition = unsafe { Composition::open(composition, c) };
    unsafe {
        drop_in_place(composition.get_unchecked_mut());
    }
}
