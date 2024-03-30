use crate::{context::Context, runner::Runner};

pub trait Element<R: Runner> {
    fn build(&self, cx: Context<R>);
}

impl<F, R: Runner> Element<R> for F
where
    F: Fn(Context<R>),
{
    fn build(&self, cx: Context<R>) {
        self(cx)
    }
}
