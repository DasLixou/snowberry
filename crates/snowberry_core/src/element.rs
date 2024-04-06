use crate::context::Context;

pub trait Element<'scope> {
    fn build(&self, cx: &mut Context<'scope, '_>);
}

impl<'scope, F> Element<'scope> for F
where
    F: Fn(&mut Context<'scope, '_>),
{
    fn build(&self, cx: &mut Context<'scope, '_>) {
        self(cx)
    }
}
