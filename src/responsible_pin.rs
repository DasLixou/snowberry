use std::{
    mem::{ManuallyDrop, MaybeUninit},
    pin::Pin,
    ptr::drop_in_place,
};

use crate::uninit::Uninit;

/// A wrapper struct over [`Pin<&mut T>`] taking full responsibility over its drop behavior.
/// This means that the pinned data itself won't be responsible for the drop, but this type here.
/// This allows to cast types and change drop behavior of the underlying data while still remaining pinned.
#[repr(transparent)]
pub struct ResponsiblePin<'life, T> {
    inner: Pin<&'life mut T>,
}

impl<'life, T> ResponsiblePin<'life, T> {
    /// Creates a new [`ResponsiblePin`].
    ///
    /// The `MaybeUninit` is only used for the actual pin data to not drop itself
    /// and serves the purpose of a [`ManuallyDrop`], but without its limitations,
    /// see https://doc.rust-lang.org/stable/core/mem/struct.ManuallyDrop.html#interaction-with-box
    /// (tho I'm not sure if this limits us...)
    ///
    /// # Safety
    ///
    /// * The caller must guarantee that the underlying MaybeUninit is initialized.
    /// * The caller must also guarantee that the returned value will not be forgotten.
    pub unsafe fn new_unchecked(inner: Pin<&'life mut MaybeUninit<T>>) -> Self {
        Self {
            inner: unsafe { inner.map_unchecked_mut(|pin| pin.assume_init_mut()) },
        }
    }

    pub fn reborrow_pin<'a>(&'a mut self) -> Pin<&'a mut T> {
        unsafe {
            let pin = &mut self.inner;
            // SAFETY: this basically clones the Pin, which should be fine as we can't use ourselves as long as the return value exists.
            let pin: Pin<&'life mut T> = core::ptr::read(pin as *mut _ as *const _);
            // SAFETY: lifetime of &mut is covariant, so making it smaller is fine
            pin
        }
    }

    /// # Safety
    ///
    /// The caller must guarantee that the drop function of the underlying type will be called when the pin gets out of scope.
    pub unsafe fn into_inner(self) -> Pin<&'life mut T> {
        // SAFETY: the caller must assure that the data under the pin get's dropped.
        let mut me = ManuallyDrop::new(self);
        let ptr = &mut me.inner;
        let pin: Pin<&'life mut T> = core::ptr::read(ptr as *mut _ as *const _);
        pin
    }
}

impl<'life, T> Drop for ResponsiblePin<'life, T> {
    fn drop(&mut self) {
        unsafe {
            let pin = &mut self.inner;
            // SAFETY: this basically clones the Pin, which should be fine as everything with the original struct is UB afterwards anyways.
            let pin: Pin<&'life mut T> = core::ptr::read(pin as *mut _ as *const _);
            let raw = pin.get_unchecked_mut();
            drop_in_place(raw);
        }
    }
}

/// # Safety
///
/// The given [`ResponsiblePin`] **must not be forgotten**.
/// Thus, calling this macro requires an explicit unsafe.
macro_rules! responsible_pin {
    (let unsafe $name:ident = $($tokens:tt)*) => {
        let __responsible_pin = ::core::pin::pin!(::std::mem::MaybeUninit::new($($tokens)*));
        let $name = unsafe { $crate::responsible_pin::ResponsiblePin::new_unchecked(__responsible_pin) };
    };
}

/*
fn main() {
    // SAFETY: we don't forget it
    responsible_pin!(let unsafe data = Uninit::uninit());
    // Commenting this out won't drop invalid data behind the MaybeUninit because the ResponsiblePin is still over MaybeUninit
    // panic!();
    let data = data.init(S("now will be dropped safely :>"));
}

struct S(&'static str);

impl Drop for S {
    fn drop(&mut self) {
        println!("{}", self.0);
    }
}
*/
