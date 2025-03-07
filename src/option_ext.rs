use std::{mem::MaybeUninit, pin::Pin};

use crate::responsible_pin::ResponsiblePin;

impl<'life, T> ResponsiblePin<'life, Option<T>> {
    pub fn make_none(self) -> Self {
        unsafe {
            let inner: &mut Option<T> = self.raw_pin().get_unchecked_mut();
            match inner {
                Some(to_drop) => core::ptr::drop_in_place(to_drop),
                None => {}
            }
            *inner = Option::None;
            ResponsiblePin::new_unchecked(Pin::new_unchecked(inner))
        }
    }

    pub fn make_some<F>(self, f: F) -> Self
    where
        F: FnOnce(Pin<&'life mut MaybeUninit<T>>),
    {
        unsafe {
            let inner: &'life mut Option<T> = self.raw_pin().get_unchecked_mut();
            match inner {
                Some(to_drop) => core::ptr::drop_in_place(to_drop),
                None => {}
            }
            let inner: &'life mut Option<MaybeUninit<T>> = &mut *(inner as *mut Option<T>).cast();
            *inner = Option::Some(MaybeUninit::uninit());
            {
                let Some(mu) = inner else { unreachable!() };
                let mu = Pin::new_unchecked(mu);
                f(mu);
            }
            let inner: &mut Option<T> = &mut *(inner as *mut Option<MaybeUninit<T>>).cast();
            ResponsiblePin::new_unchecked(Pin::new_unchecked(inner))
        }
    }
}
