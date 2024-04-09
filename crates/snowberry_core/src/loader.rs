use std::cell::RefCell;

use crate::context::Context;

pub struct LoaderHandle<'scope> {
    is_enabled: &'scope RefCell<bool>,
}

// TODO: better name - I want to use loader for async stuff later
pub fn loader<'scope: 'sub, 'sub>(
    cx: &'sub mut Context<'scope, '_>,
    element: impl Fn(&mut Context<'sub, '_>),
) -> LoaderHandle<'scope> {
    let is_enabled = cx.store(RefCell::new(true));

    cx.sub_scope(element);

    LoaderHandle { is_enabled }
}
