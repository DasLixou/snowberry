use std::marker::PhantomData;

/// Typically implemented on ZST type or a FnOnce wrapper when creation holds variables
pub trait Composable<'life>: 'life {
    type Store: Sized + 'life;

    fn compose(self, store: &'life mut Self::Store);
}

/*
/// Reify util to convert generics of a trait into assoicated types of a simple trait.
pub mod reify {
    use std::marker::PhantomData;

    pub trait IntoReified<S>: Sized {
        fn reify(self) -> Described<S, Self> {
            Described {
                inner: self,
                phantom: PhantomData,
            }
        }
    }

    pub struct Described<S, R: IntoReified<S>> {
        inner: R,
        phantom: PhantomData<S>,
    }

    pub trait Reified {
        type S;
    }

    impl<S, R: IntoReified<S>> Reified for Described<S, R> {
        type S = S;
    }
}

impl<S, F> reify::IntoReified<S> for F where F: FnOnce(&mut S) {}*/

pub fn make_composable<'life, F, S>(closure: F) -> impl Composable<'life>
where
    F: FnOnce(&'life mut S) + 'life,
    S: 'life,
{
    struct MyComposable<'life, F, S> {
        closure: F,
        phantom: PhantomData<&'life S>,
    }
    impl<'life, F, S> Composable<'life> for MyComposable<'life, F, S>
    where
        F: FnOnce(&'life mut S) + 'life,
        S: 'life,
    {
        type Store = S;

        fn compose(self, store: &'life mut Self::Store) {
            (self.closure)(store);
        }
    }
    MyComposable::<'life, F, S> {
        closure,
        phantom: PhantomData,
    }
}

#[macro_export]
macro_rules! composable {
    ($($tokens:tt)*) => {{
        ::snowberry::composable::make_composable($($tokens)*)
    }};
}
