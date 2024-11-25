use std::mem::MaybeUninit;

use crate::store::Store;

pub struct ReactionChain<'scope, C: Reaction> {
    inner: &'scope mut MaybeUninit<C>,
}

impl<'scope, C: Reaction> ReactionChain<'scope, C> {
    pub fn on<Rest>(store: Store<'scope, (Rest, C)>) -> (Store<'scope, Rest>, Self, &C) {
        let (store, me) = store.split_off();
        let x = unsafe { &*me.as_ptr() }; // TODO: still unsafe and multiple borrows >:C
        (store, ReactionChain { inner: me }, x)
    }
}

impl<'scope, Rest: Reaction + 'scope, R: Reaction + 'scope> ReactionChain<'scope, (Rest, R)> {
    #[must_use]
    fn split_off(self) -> (ReactionChain<'scope, Rest>, &'scope mut MaybeUninit<R>) {
        unsafe {
            (
                ReactionChain {
                    inner: &mut *(&raw mut (*self.inner.as_mut_ptr()).0).cast(),
                },
                &mut *(&raw mut (*self.inner.as_mut_ptr()).1).cast(),
            )
        }
    }

    #[must_use]
    pub fn react(self, reaction: R) -> ReactionChain<'scope, Rest> {
        let (chain, slot) = self.split_off();
        slot.write(reaction);
        chain
    }
}

impl ReactionChain<'_, ()> {
    pub fn end(self) {
        let _ = self;
    }
}

pub trait Reaction {
    fn invoke(&self);
}

impl Reaction for () {
    fn invoke(&self) {}
}

impl<A: Reaction, B: Reaction> Reaction for (A, B) {
    fn invoke(&self) {
        self.0.invoke();
        self.1.invoke();
    }
}

impl<F> Reaction for F
where
    F: Fn(),
{
    fn invoke(&self) {
        (self)()
    }
}
