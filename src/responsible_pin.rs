use std::{mem::MaybeUninit, pin::Pin};

/// A wrapper struct over [`Pin<&mut T>`] taking full responsibility over its drop behavior.
/// This means that the pinned data itself won't be responsible for the drop, but this type here.
/// This allows to cast types and change drop behavior of the underlying data while still remaining pinned.
#[repr(transparent)]
pub struct ResponsiblePin<'life, T> {
    // the pinned MaybeUninit here is always initialized, it's just more safe than ManuallyDrop when it comes to moves.
    inner: Pin<&'life mut MaybeUninit<T>>,
}

impl<'life, T> ResponsiblePin<'life, T> {
    pub unsafe fn new_unchecked(inner: Pin<&'life mut MaybeUninit<T>>) -> Self {
        Self { inner }
    }

    pub fn reborrow_pin<'a>(&'a mut self) -> Pin<&'a mut T> {
        unsafe {
            let pin = &mut self.inner;
            // SAFETY: this basically clones the Pin, which should be fine as we can't use ourselves as long as the return value exists.
            // TODO: read or read_unaligned?
            let pin: Pin<&'life mut MaybeUninit<T>> =
                core::ptr::read_unaligned(pin as *mut _ as *const _);
            // SAFETY: inner is always initialized, see comment on field
            // SAFETY: lifetime of &mut is covariant, so making it smaller is fine
            let raw = pin.get_unchecked_mut().as_mut_ptr();
            Pin::new_unchecked(&mut *raw)
        }
    }
}

impl<'life, T> Drop for ResponsiblePin<'life, T> {
    fn drop(&mut self) {
        unsafe {
            let pin = &mut self.inner;
            // SAFETY: this basically clones the Pin, which should be fine as everything with the original struct is UB afterwards anyways.
            // TODO: read or read_unaligned?
            let pin: Pin<&'life mut MaybeUninit<T>> =
                core::ptr::read_unaligned(pin as *mut _ as *const _);
            let raw = pin.get_unchecked_mut();
            // SAFETY: inner is always initialized, see comment on field
            raw.assume_init_drop();
        }
    }
}

macro_rules! responsible_pin {
    (let $name:ident = $($tokens:tt)*) => {
        let __responsible_pin = ::core::pin::pin!(::std::mem::MaybeUninit::new($($tokens)*));
        let $name = unsafe {
            $crate::responsible_pin::ResponsiblePin::new_unchecked(__responsible_pin)
        };
    };
}

/*
fn main() {
    responsible_pin!(let data = MaybeUninit::uninit());
    // Commenting this out won't drop invalid data behind the MaybeUninit because the ResponsiblePin is still over MaybeUninit
    // panic!();
    let data = write_util(data, S("now will be dropped safely :>"));
}

fn write_util<'a, T>(
    mut uninit: ResponsiblePin<'a, MaybeUninit<T>>,
    val: T,
) -> ResponsiblePin<'a, T> {
    unsafe {
        uninit.reborrow_pin().get_unchecked_mut().write(val);
        // SAFETY: MaybeUninit is transparent
        core::mem::transmute(uninit)
    }
}

struct S(&'static str);

impl Drop for S {
    fn drop(&mut self) {
        println!("{}", self.0);
    }
}
*/
