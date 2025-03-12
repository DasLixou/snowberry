use std::marker::PhantomData;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct LensTable<Down: Copy> {
    down: Down,
    cur: *mut (),
}

impl LensTable<()> {
    pub(crate) fn root(ptr: *mut ()) -> LensTable<()> {
        LensTable { down: (), cur: ptr }
    }
}

impl<Down: Copy> LensTable<Down> {
    pub(crate) fn extend(&self, ptr: *mut ()) -> LensTable<Self> {
        LensTable {
            down: *self,
            cur: ptr,
        }
    }

    pub(crate) fn offset() -> usize {
        core::mem::offset_of!(Self, cur)
    }

    pub(crate) unsafe fn retrieve(&self, offset: usize) -> *mut () {
        unsafe {
            let me = self as *const Self as *const *mut ();
            let pos = me.byte_add(offset);
            core::ptr::read(pos)
        }
    }
}

// TODO: add lifetime
pub struct Lens<T> {
    pub(crate) table_offset: usize,
    pub(crate) store_offset: usize,
    pub(crate) phantom: PhantomData<T>,
}
