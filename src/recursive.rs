use std::mem::ManuallyDrop;

/// Recursive struct helper for guaranteeing layout and drop order.
// TODO: make sure that layout for nesting is correct and guaranteed
#[repr(C)]
pub struct Rec<D, T> {
    down: ManuallyDrop<D>,
    val: ManuallyDrop<T>,
}
impl<D, T> Rec<D, T> {
    pub fn split(&mut self) -> (&mut D, &mut T) {
        (&mut self.down, &mut self.val)
    }

    pub fn offset() -> usize {
        core::mem::offset_of!(Self, val)
    }
}
impl<D, T> Drop for Rec<D, T> {
    fn drop(&mut self) {
        unsafe {
            // We need to drop val before down so drop order is consistent with how rust drops variables in functions.
            ManuallyDrop::drop(&mut self.val);
            ManuallyDrop::drop(&mut self.down);
        }
    }
}
