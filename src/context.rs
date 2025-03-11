use crate::{
    composable_::Composable, composition::Composition, ext_stack::ExtStackRef,
    lens_table::LensTable, recursive::Rec,
};

pub struct Context<'life, Init, Down: Copy> {
    pub store: ExtStackRef<'life, Init>,
    pub lens_table: LensTable<Down>,
}

impl<'life, Init, Down: Copy> Context<'life, Init, Down> {
    pub fn store<T>(self, val: T) -> Context<'life, Rec<Init, T>, Down> {
        Context {
            store: self.store.store(val),
            lens_table: self.lens_table,
        }
    }

    pub fn compose<'comp, C>(
        self,
        composable: C,
    ) -> Context<'life, Rec<Init, Composition<'comp, C>>, Down>
    where
        C: Composable<'comp, Down = LensTable<Down>>,
        Down: 'comp,
    {
        let (cx, composition) = Composition::open(self, composable);
        Context {
            store: cx.store.store(composition),
            lens_table: cx.lens_table,
        }
    }
}
