use crate::{
    composable_::Composable, context::Context, extend_for, lens_table::LensTable, scope::Scope,
};

pub struct Composition<'scope, C: Composable<'scope>> {
    stored: C::Store,
}

impl<'scope, C: Composable<'scope>> Composition<'scope, C> {
    pub fn root<Env: Copy + 'scope>(scope: Scope<'scope>, composable: C, environment: Env) -> Self
    where
        C: Composable<'scope>,
    {
        Composition {
            stored: extend_for!(|ext_ref| {
                let lens_table = unsafe { LensTable::root(ext_ref.ptr_mut()) };
                let context = Context {
                    scope,
                    store: ext_ref,
                    lens_table,
                    environment,
                };
                let context = composable.compose(context);
                context.store
            }),
        }
    }

    pub fn open<'cx, I, D: Copy + 'cx, E: Copy + 'cx>(
        scope: Scope<'scope>,
        cx: Context<'scope, 'cx, I, D, E>,
        composable: C,
    ) -> (Context<'scope, 'cx, I, D, E>, Self)
    where
        C: Composable<'scope>,
    {
        let stored = extend_for!(|ext_ref| {
            let lens_table = unsafe { cx.lens_table.extend(ext_ref.ptr_mut()) };
            let context = Context {
                scope,
                store: ext_ref,
                lens_table,
                environment: cx.environment,
            };
            let context = composable.compose(context);
            context.store
        });
        (cx, Composition { stored })
    }
}
