use crate::context::Context;

pub trait Element {
    fn build(&self, cx: &mut Context<'_, '_>);
}

impl<F> Element for F
where
    F: Fn(&mut Context),
{
    fn build(&self, cx: &mut Context<'_, '_>) {
        self(cx)
    }
}
