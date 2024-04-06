use std::marker::PhantomData;

use slotmap::SlotMap;

use crate::{
    element::Element,
    resource::Resources,
    scope::{Scope, ScopeKey, ScopeLife},
};

pub struct Context<'scope, 'call> {
    pub resources: &'call mut Resources,
    pub scopes: &'call mut SlotMap<ScopeKey, Scope>,
    pub scope: ScopeKey,
    pub life: ScopeLife<'scope>,
}

impl<'scope, 'call> Context<'scope, 'call> {
    pub fn store<T: 'static>(&mut self, val: T) -> &'scope T {
        self.scopes[self.scope].store(self.life, val)
    }

    pub fn sub_scope<'sub, E>(&'sub mut self, e: E)
    where
        E: Element<'sub>,
        'scope: 'sub,
    {
        let key = self.scopes.insert(Scope::new());
        self.scopes[self.scope].sub_scopes.push(key);
        e.build(&mut Context {
            resources: &mut self.resources,
            scope: key,
            scopes: &mut self.scopes,
            life: ScopeLife(PhantomData),
        });
    }

    // TODO: what happens when I do {store; kill itself; read that stored val}?
    // maybe this should only be allowed to close itself and be consuming? oh no... can't be consuming.. shit
    pub fn close_scope(&mut self, scope: ScopeKey) {
        if let Some(scope) = self.scopes.remove(scope) {
            for sub_scope in &scope.sub_scopes {
                self.close_scope(*sub_scope);
            }
            drop(scope); // explicit in the case that I want to do context-needed drop later on
        }
    }
}
