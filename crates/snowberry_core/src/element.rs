use crate::context::Context;

pub trait Element {
    fn build(&self, cx: Context<'_, '_>);
}

impl<F> Element for F
where
    F: Fn(Context),
{
    fn build(&self, cx: Context<'_, '_>) {
        self(cx)
    }
}
