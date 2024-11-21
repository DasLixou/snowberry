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

pub fn split_off<Rest, T>(
    tuple: &mut MaybeUninit<(Rest, T)>,
) -> (&mut MaybeUninit<Rest>, &mut MaybeUninit<T>) {
    unsafe {
        (
            &mut *(&raw mut (*tuple.as_mut_ptr()).0).cast(),
            &mut *(&raw mut (*tuple.as_mut_ptr()).1).cast(),
        )
    }
}

pub fn store_val<Rest, T>(
    store: &mut MaybeUninit<(Rest, T)>,
    val: T,
) -> (&mut MaybeUninit<Rest>, &mut T) {
    let (store, slot) = split_off::<Rest, T>(store);
    (store, slot.write(val))
}

pub fn end(tuple: &mut MaybeUninit<()>) {
    let _ = tuple;
}
