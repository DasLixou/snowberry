use crate::store::Store;

pub trait Composable<'s, S: 's> {
    fn compose(self, store: Store<'s, S>);
}

impl<'s, F, S: 's> Composable<'s, S> for F
where
    F: FnOnce(Store<'s, S>),
{
    fn compose(self, store: Store<'s, S>) {
        (self)(store);
    }
}
