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
}
