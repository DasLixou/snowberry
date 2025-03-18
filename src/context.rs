use std::{marker::PhantomData, ops::Index};

use crate::{
    composable_::Composable,
    composition::Composition,
    environment::WithEnv,
    ext_stack::ExtStackRef,
    lens_table::{Lens, LensTable},
    recursive::Rec,
    scope::Scope,
};

pub struct Context<'s, 'life, Init, Down: Copy, Env> {
    pub scope: Scope<'s>,
    pub store: ExtStackRef<'life, Init>,
    pub lens_table: LensTable<Down>,
    pub environment: Env,
}

impl<'s, 'life, Init, Down: Copy, Env: Copy> Context<'s, 'life, Init, Down, Env> {
    pub fn store<T: 'life>(
        self,
        val: T,
    ) -> (Context<'s, 'life, Rec<Init, T>, Down, Env>, Lens<'life, T>) {
        let (store, offset) = self.store.store(val);
        (
            Context {
                scope: self.scope,
                store,
                lens_table: self.lens_table,
                environment: self.environment,
            },
            Lens {
                table_offset: LensTable::<Down>::offset(),
                store_offset: offset,
                phantom: PhantomData,
            },
        )
    }

    pub fn compose<C>(
        self,
        composable: C,
    ) -> Context<'s, 'life, Rec<Init, Composition<'s, C>>, Down, Env>
    where
        C: Composable<'s>,
        Down: 'life,
        Env: 'life,
    {
        let (cx, composition) =
            Scope::<'s>::sub(|scope| Composition::open(scope, self, composable));
        cx.store(composition).0
    }
}

impl<'life, Init, Down: Copy, Env, T> Index<Lens<'life, T>>
    for Context<'_, 'life, Init, Down, Env>
{
    type Output = T;

    fn index(&self, index: Lens<'life, T>) -> &Self::Output {
        unsafe {
            // TODO: verify ptr with that provenance thing
            let store = self.lens_table.retrieve(index.table_offset);
            &*(store.byte_add(index.store_offset) as *const T)
        }
    }
}

impl<'life, Init, Down: Copy, Env> Context<'_, 'life, Init, Down, Env> {
    pub fn env<T>(&self) -> T
    where
        Env: WithEnv<T>,
    {
        self.environment.get()
    }
}
