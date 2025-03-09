use crate::{
    ext_stack::ExtStackRef,
    recursive::{Rec, Recursive},
};

pub struct Context<'life, Init, Todo> {
    pub store: ExtStackRef<'life, Init, Todo>,
}

impl<'life, Init, Todo: Recursive> Context<'life, Init, Todo> {
    pub fn store(self, val: Todo::Pop) -> Context<'life, Rec<Init, Todo::Pop>, Todo::Remainder> {
        Context {
            store: self.store.store(val),
        }
    }
}
