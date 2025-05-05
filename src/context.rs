use crate::composable_::Composable;

pub struct Context<'cx, Env> {
    pub env: &'cx Env,
}

impl<'cx, Env> Context<'cx, Env> {
    pub fn compose(self, c: impl Composable<Env>) -> Context<'cx, Env> {
        c.compose(Context { env: self.env });
        self
    }
}
