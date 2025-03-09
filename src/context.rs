use crate::{ext_stack::ExtStackRef, generics_stack::RecursiveTuple};

pub struct Context<'life, Init, Todo> {
    pub store: ExtStackRef<'life, Init, Todo>,
}

impl<'life, Init, Todo: RecursiveTuple> Context<'life, Init, Todo> {
    pub fn store(self, val: Todo::Pop) -> Context<'life, (Init, Todo::Pop), Todo::Remainder> {
        Context {
            store: self.store.store(val),
        }
    }
}
