use std::marker::PhantomData;

use crate::context::Context;

/// Typically implemented on ZST type or a FnOnce wrapper when creation holds variables
pub trait Composable<'life>: 'life {
    type Store: Sized + 'life;
    type Down: Copy;

    fn compose<'cx>(
        self,
        cx: Context<'cx, (), Self::Store, Self::Down>,
    ) -> Context<'cx, Self::Store, (), Self::Down>;
}

pub fn make_composable<'life, F, S, D>(closure: F) -> impl Composable<'life, Down = D>
where
    for<'cx> F: FnOnce(Context<'cx, (), S, D>) -> Context<'cx, S, (), D> + 'life,
    S: 'life,
    D: Copy + 'life,
{
    struct MyComposable<'life, F, S, D> {
        closure: F,
        phantom: PhantomData<&'life (S, D)>,
    }
    impl<'life, F, S, D: Copy> Composable<'life> for MyComposable<'life, F, S, D>
    where
        for<'cx> F: FnOnce(Context<'cx, (), S, D>) -> Context<'cx, S, (), D> + 'life,
        S: 'life,
    {
        type Store = S;
        type Down = D;

        fn compose<'cx>(
            self,
            cx: Context<'cx, (), Self::Store, D>,
        ) -> Context<'cx, Self::Store, (), D> {
            (self.closure)(cx)
        }
    }
    MyComposable::<F, S, D> {
        closure,
        phantom: PhantomData,
    }
}

#[macro_export]
macro_rules! composable {
    ($($tokens:tt)*) => {{
        ::snowberry::composable_::make_composable($($tokens)*)
    }};
}
