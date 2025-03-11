use crate::{
    composable::Composable, context::Context, ext_stack::ExtStack, lens_table::LensTable,
    scope::Scope,
};

pub struct Composition<'scope, C: Composable<'scope>> {
    scope: Scope<'scope>,
    stored: ExtStack<C::Store>,
}

impl<'scope, C: Composable<'scope>> Composition<'scope, C> {
    pub fn root(composable: C) -> Self
    where
        C: Composable<'scope, Down = ()>,
    {
        let (scope, stored) = Scope::open(|_scope| {
            ExtStack::extend_for(|ext_ref| {
                let lens_table = unsafe { LensTable::root(ext_ref.ptr_mut()) };
                let context = Context {
                    store: ext_ref,
                    lens_table,
                };
                let context = composable.compose(context);
                context.store
            })
        });
        Composition { scope, stored }
    }

    pub fn open<'cx, I, T, D: Copy + 'scope>(
        cx: Context<'cx, I, T, D>,
        composable: C,
    ) -> (Context<'cx, I, T, D>, Self)
    where
        C: Composable<'scope, Down = LensTable<D>>,
    {
        let (scope, stored) = Scope::open(move |_scope| {
            ExtStack::extend_for(|ext_ref| {
                let lens_table = unsafe { cx.lens_table.extend(ext_ref.ptr_mut()) };
                let context = Context {
                    store: ext_ref,
                    lens_table,
                };
                let context = composable.compose(context);
                context.store
            })
        });
        (cx, Composition { scope, stored })
    }
}
