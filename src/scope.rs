use std::marker::PhantomData;

type InvariantLifetime<'life> = PhantomData<fn(&'life ()) -> &'life ()>;

pub struct Scope<'scope> {
    lifetime: InvariantLifetime<'scope>,
}

impl<'scope> Scope<'scope> {
    pub fn new() -> Self {
        Self {
            lifetime: PhantomData,
        }
    }

    pub fn open<F, T>(sub: F) -> (Self, T)
    where
        F: FnOnce(&'_ mut Self) -> T + 'scope,
    {
        let mut scope = Scope::new();
        let t = sub(&mut scope);
        (scope, t)
    }

    pub fn sub<'sub, F>(&self, sub: F) -> Scope<'sub>
    where
        F: FnOnce(&'_ mut Scope<'sub>) + 'sub,
        'scope: 'sub,
        'sub: 'scope,
    {
        let mut scope = Scope::new();
        sub(&mut scope);
        scope
    }
}

#[doc(hidden)]
#[allow(dead_code)]
mod comp_test {
    /// ```compile_fail,E0521
    /// use snowberry::prelude::Scope;
    ///
    /// Scope::open(|a| {
    ///     Scope::open(|b| {
    ///         same_brand(a, b);
    ///     });
    /// });
    ///
    /// fn same_brand<'a>(a: &mut Scope<'a>, b: &mut Scope<'a>) {
    ///     let _ = a;
    ///     let _ = b;
    /// }
    /// ```
    fn scope_cant_escape() {}
}
