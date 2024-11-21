use std::mem::MaybeUninit;

pub trait Composable<'s, Store: 's> {
    fn compose(self, store: &'s mut MaybeUninit<Store>);
}

impl<'s, F, S: 's> Composable<'s, S> for F
where
    F: FnOnce(&'s mut MaybeUninit<S>),
{
    fn compose(self, store: &'s mut MaybeUninit<S>) {
        (self)(store);
    }
}

// store utils

pub fn split_off<T, Rest>(
    tuple: &mut MaybeUninit<(T, Rest)>,
) -> (&mut MaybeUninit<T>, &mut MaybeUninit<Rest>) {
    unsafe {
        (
            &mut *(&raw mut (*tuple.as_mut_ptr()).0).cast(),
            &mut *(&raw mut (*tuple.as_mut_ptr()).1).cast(),
        )
    }
}

pub fn store_val<T, Rest>(
    val: T,
    store: &mut MaybeUninit<(T, Rest)>,
) -> (&mut T, &mut MaybeUninit<Rest>) {
    let (slot, store) = split_off::<T, Rest>(store);
    (slot.write(val), store)
}

pub fn end(tuple: &mut MaybeUninit<()>) {
    let _ = tuple;
}
