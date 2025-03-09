use crate::{composable::Composable, context::Context, ext_stack::ExtStack, scope::Scope};

pub struct Composition<'scope, C: Composable<'scope>> {
    scope: Scope<'scope>,
    stored: ExtStack<C::Store>,
}

impl<'scope, C: Composable<'scope>> Composition<'scope, C> {
    pub fn open(composable: C) -> Self {
        let (scope, stored) = Scope::open(|_scope| {
            ExtStack::extend_for(|ext_ref| {
                let context = Context { store: ext_ref };
                let context = composable.compose(context);
                context.store
            })
        });
        Composition { scope, stored }
    }
}
