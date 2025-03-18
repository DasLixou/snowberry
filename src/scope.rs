use std::marker::PhantomData;

type InvariantLifetime<'life> = PhantomData<fn(&'life ()) -> &'life ()>;

pub struct Scope<'scope> {
    lifetime: InvariantLifetime<'scope>,
}

impl<'scope> Scope<'scope> {
    pub fn new<F, T>(sub: F) -> T
    where
        for<'new_scope> F: FnOnce(Scope<'new_scope>) -> T,
    {
        let scope = Scope {
            lifetime: PhantomData,
        };
        sub(scope)
    }

    pub fn sub<F, T>(sub: F) -> T
    where
        F: FnOnce(Scope<'scope>) -> T,
    {
        let scope = Scope {
            lifetime: PhantomData,
        };
        sub(scope)
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
