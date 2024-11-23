use std::{marker::PhantomData, mem::MaybeUninit};

use crate::store::Store;

pub struct Reactive<'scope, T, Chain> {
    val: T,
    reaction_chain: &'scope Chain,
}

impl<'scope, T, Chain> Reactive<'scope, T, Chain> {
    pub fn update<F>(&mut self, f: F)
    where
        F: FnOnce(&mut T),
    {
        f(&mut self.val)
    }
}

pub fn reactive<'scope, Rest, T, Chain>(
    store: Store<'scope, ((Rest, Reactive<'scope, T, Chain>), Chain)>,
    val: T,
) -> (
    Store<'scope, Rest>,
    &'scope Chain,
    &'scope mut Reactive<'scope, T, Chain>,
) {
    let (store, chain) = store.split_off();
    let chain = unsafe { chain.assume_init_ref() };
    let (store, reactive) = store.split_off();
    let reactive = reactive.write(Reactive {
        val,
        reaction_chain: chain,
    });
    (store, chain, reactive)
}

// chain utils

struct Listener<const F: i32> {}

pub fn chain_end(chain: &'_ ()) {
    let _ = chain;
}
