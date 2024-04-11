use crate::{context::Context, scope::ScopeKey};

pub struct EventStation<E: Clone + 'static> {
    pub listeners: Vec<(ScopeKey, Box<dyn Listener<E>>)>,
}

impl<E: Clone + 'static> EventStation<E> {
    pub fn new() -> Self {
        Self { listeners: vec![] }
    }

    pub fn listen<L>(&mut self, scope: ScopeKey, l: L)
    where
        L: Listener<E> + 'static,
    {
        self.listeners.push((scope, Box::new(l)));
    }

    pub fn to_erased(&self, event: E) -> ErasedEventStation {
        ErasedEventStation {
            listener_calls: self
                .listeners
                .iter()
                .map(|(scope, l)| {
                    let b = l.cloned_box();
                    let e = event.clone();
                    let erased = ErasedListenerCall {
                        call: Box::new(move |cx| {
                            b.run(e, cx);
                        }),
                    };
                    (*scope, erased)
                })
                .collect(),
        }
    }
}

impl<E: Clone> Clone for EventStation<E> {
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
    fn run(&self, event: E, cx: &mut Context<'_, '_>);
    fn cloned_box(&self) -> Box<dyn Listener<E>>;
}

impl<E, F> Listener<E> for F
where
    F: Fn(E, &mut Context<'_, '_>) + Clone + 'static,
{
    fn run(&self, event: E, cx: &mut Context<'_, '_>) {
        self(event, cx);
    }

    fn cloned_box(&self) -> Box<dyn Listener<E>> {
        Box::new(self.clone())
    }
}

pub struct ErasedEventStation {
    pub listener_calls: Vec<(ScopeKey, ErasedListenerCall)>,
}

pub struct ErasedListenerCall {
    call: Box<dyn FnOnce(&mut Context)>,
}

impl ErasedListenerCall {
    pub fn run(self, cx: &mut Context<'_, '_>) {
        (self.call)(cx);
    }
}
