use slotmap::SlotMap;

use crate::{
    element::Element,
    resource::Resources,
    scope::{Scope, ScopeKey},
};

pub struct Context<'scope, 'call> {
    pub resources: &'call mut Resources,
    pub scopes: &'scope mut SlotMap<ScopeKey, Scope>,
    pub scope: ScopeKey,
}

impl<'scope, 'call> Context<'scope, 'call> {
    pub fn store<T: 'static>(&mut self, val: T) {
        self.scopes[self.scope].store(val);
    }

    pub fn sub_scope<E>(&mut self, e: E)
    where
        E: Element,
    {
        let key = self.scopes.insert(Scope::new());
        self.scopes[self.scope].sub_scopes.push(key);
        e.build(&mut Context {
            resources: &mut self.resources,
            scope: key,
            scopes: &mut self.scopes,
        });
    }
}
