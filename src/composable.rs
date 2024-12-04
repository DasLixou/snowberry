use crate::store::Store;

/// Typically implemented on ZST type or a FnOnce wrapper when creation holds variables
pub trait Composable<'life>: 'life {
    type Store: Sized + 'life;

    fn compose(self, store: Store<'life, Self::Store>);
}
