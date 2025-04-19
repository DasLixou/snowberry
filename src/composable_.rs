use crate::context::Context;

pub trait Composable<Env> {
    fn compose<'cx>(self, cx: Context<'cx, Env>);
}

#[macro_export]
macro_rules! composable {
    ($($tokens:tt)*) => {{
        fn make_composable<F, Env>(f: F) -> impl $crate::composable_::Composable<Env>
        where
            F: FnOnce($crate::context::Context<Env>)
        {
            struct MyComposable<F, Env>(F, ::core::marker::PhantomData<Env>);
            impl<F, Env> $crate::composable_::Composable<Env> for MyComposable<F, Env>
            where
                F: FnOnce($crate::context::Context<Env>)
            {
                fn compose<'cx>(self, cx: $crate::context::Context<'cx, Env>) {
                    (self.0)(cx);
                }
            }
            MyComposable(f, ::core::marker::PhantomData)
        }
        make_composable($($tokens)*)
    }};
}
