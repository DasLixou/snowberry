use std::marker::PhantomData;

use crate::composable_::Composable;

pub struct Context<'cx, Env> {
    pub env: PhantomData<&'cx Env>,
}

impl<'cx, Env> Context<'cx, Env> {
    pub fn compose(self, c: impl Composable<Env>) -> Context<'cx, Env> {
        c.compose(Context { env: PhantomData });
        self
    }
}
