use crate::{context::Context, scope::ScopeKey};

pub struct EventStation<E> {
    pub listeners: Vec<(ScopeKey, Box<dyn Listener<E>>)>,
}

impl<E> EventStation<E> {
    pub fn new() -> Self {
        Self { listeners: vec![] }
    }

    pub fn listen<L>(&mut self, scope: ScopeKey, l: L)
    where
        L: Listener<E> + 'static,
    {
        self.listeners.push((scope, Box::new(l)));
    }
}

impl<E> Clone for EventStation<E> {
    fn clone(&self) -> Self {
        Self {
            listeners: self
                .listeners
                .iter()
                .map(|(scope, l)| (*scope, l.cloned_box()))
                .collect(),
        }
    }
}

pub trait Listener<E> {
    fn run(&self, event: &E, cx: &mut Context<'_, '_>);
    fn cloned_box(&self) -> Box<dyn Listener<E>>;
}

impl<E, F> Listener<E> for F
where
    F: Fn(&E, &mut Context<'_, '_>) + Clone + 'static,
{
    fn run(&self, event: &E, cx: &mut Context<'_, '_>) {
        self(event, cx);
    }

    fn cloned_box(&self) -> Box<dyn Listener<E>> {
        Box::new(self.clone())
    }
}
