use std::mem::MaybeUninit;

use crate::responsible_pin::ResponsiblePin;

#[repr(transparent)]
pub struct Uninit<T> {
    inner: MaybeUninit<T>,
}

impl<T> Uninit<T> {
    pub fn uninit() -> Self {
        Self {
            inner: MaybeUninit::uninit(),
        }
    }
}

impl<'life, T> ResponsiblePin<'life, Uninit<T>> {
    /// # Safety reminder
    ///
    /// The returned value still **must not be forgotten**.
    pub fn init(self, val: T) -> ResponsiblePin<'life, T> {
        unsafe {
            // SAFETY: Uninit doesn't have a drop function.
            let pin = self.raw_pin();
            // SAFETY: we don't move the underlying data.
            let pin = pin.map_unchecked_mut(|p| p.inner.write(val));
            // SAFETY: it is initialized, the forget promise is already upheld from the old type.
            ResponsiblePin::new_unchecked(pin)
        }
    }

    pub unsafe fn map_uninit<F, U>(self, f: F) -> ResponsiblePin<'life, Uninit<U>>
    where
        F: FnOnce(*mut T) -> *mut U,
    {
        self.map(|unpin| &mut *(f(unpin.inner.as_mut_ptr())).cast::<Uninit<_>>())
    }
}
