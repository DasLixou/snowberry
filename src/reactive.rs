use crate::{
    reaction_chain::{Reaction, ReactionChain},
    store::Store,
};

pub struct Reactive<'scope, T, Chain: Reaction> {
    val: T,
    reaction_chain: &'scope Chain,
}

impl<'scope, T, Chain: Reaction> Reactive<'scope, T, Chain> {
    pub fn update<F>(&mut self, f: F)
    where
        F: FnOnce(&mut T),
    {
        f(&mut self.val);
        self.reaction_chain.invoke();
    }
}

pub fn reactive<'scope, Rest, T, Chain: Reaction>(
    store: Store<'scope, ((Rest, Reactive<'scope, T, Chain>), Chain)>,
    val: T,
) -> (
    Store<'scope, Rest>,
    ReactionChain<'scope, Chain>,
    &'scope mut Reactive<'scope, T, Chain>,
) {
    let (store, chain) = store.split_off();
    let (store, reactive) = store.split_off();
    let reactive = reactive.write(Reactive {
        val,
        reaction_chain: unsafe { &*chain.as_ptr() }, // TODO: that is bad unsafe stuff, chain shouldn't live for 'scope
    });
    let chain = ReactionChain { inner: chain };
    (store, chain, reactive)
}
