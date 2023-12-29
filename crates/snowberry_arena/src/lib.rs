use std::cell::RefCell;

use bumpalo::Bump;

pub struct DropArena {
    inner: Bump,
    drops: RefCell<Vec<(*mut (), fn(*mut ()))>>,
}

impl DropArena {
    pub fn new() -> Self {
        Self {
            inner: Bump::new(),
            drops: RefCell::new(Vec::new()),
        }
    }

    pub fn alloc<T>(&self, val: T) -> &mut T {
        let val = self.inner.alloc(val);
        self.drops
            .borrow_mut()
            .push((val as *mut T as _, erased_drop::<T>));
        val
    }
}

fn erased_drop<T>(erased_ptr: *mut ()) {
    unsafe { core::ptr::drop_in_place(erased_ptr.cast::<T>()) }
}
