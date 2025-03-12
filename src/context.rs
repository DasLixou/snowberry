use std::{marker::PhantomData, ops::Index};

use crate::{
    composable_::Composable,
    composition::Composition,
    ext_stack::ExtStackRef,
    lens_table::{Lens, LensTable},
    recursive::Rec,
};

pub struct Context<'life, Init, Down: Copy> {
    pub store: ExtStackRef<'life, Init>,
    pub lens_table: LensTable<Down>,
}

impl<'life, Init, Down: Copy> Context<'life, Init, Down> {
    pub fn store<T>(self, val: T) -> (Context<'life, Rec<Init, T>, Down>, Lens<T>) {
        let (store, offset) = self.store.store(val);
        (
            Context {
                store,
                lens_table: self.lens_table,
            },
            Lens {
                table_offset: LensTable::<Down>::offset(),
                store_offset: offset,
                phantom: PhantomData,
            },
        )
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
        cx.store(composition).0
    }
}

impl<'life, Init, Down: Copy, T> Index<Lens<T>> for Context<'life, Init, Down> {
    type Output = T;

    fn index(&self, index: Lens<T>) -> &Self::Output {
        unsafe {
            // TODO: verify ptr with that provenance thing
            let store = self.lens_table.retrieve(index.table_offset);
            &*(store.byte_add(index.store_offset) as *const T)
        }
    }
}
