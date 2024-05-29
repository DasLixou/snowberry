use crate::{context::Context, ConstructId};

pub trait InitElement {
    fn exec(&self, cx: &mut Context) -> ConstructId;
}

impl<F, T: 'static> InitElement for F
where
    F: Fn(&mut Context) -> T,
{
    fn exec(&self, cx: &mut Context) -> ConstructId {
        let construct = self(cx);
        let construct = Box::new(construct);
        cx.constructs.insert(construct)
    }
}
