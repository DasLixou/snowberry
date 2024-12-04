use crate::store::Store;

pub trait Composable<'scope, S: 'scope>: 'scope {
    fn compose(self, store: Store<'scope, S>);
}

impl<'scope, S: 'scope, F> Composable<'scope, S> for F
where
    F: FnOnce(Store<'scope, S>) + 'scope,
{
    fn compose(self, store: Store<'scope, S>) {
        (self)(store);
    }
}
