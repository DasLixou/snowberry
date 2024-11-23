use std::mem::MaybeUninit;

pub struct ReactionChain<'scope, C: Reaction> {
    pub(crate) inner: &'scope mut MaybeUninit<C>,
}

impl<'scope, Rest: Reaction + 'scope, R: Reaction + 'scope> ReactionChain<'scope, (Rest, R)> {
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
