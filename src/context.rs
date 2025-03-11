use crate::{
    ext_stack::ExtStackRef,
    lens_table::LensTable,
    recursive::{Rec, Recursive},
};

pub struct Context<'life, Init, Todo, Down: Copy> {
    pub store: ExtStackRef<'life, Init, Todo>,
    pub lens_table: LensTable<Down>,
}

impl<'life, Init, Todo: Recursive, Down: Copy> Context<'life, Init, Todo, Down> {
    pub fn store(
        self,
        val: Todo::Pop,
    ) -> Context<'life, Rec<Init, Todo::Pop>, Todo::Remainder, Down> {
        Context {
            store: self.store.store(val),
            lens_table: self.lens_table,
        }
    }
}
