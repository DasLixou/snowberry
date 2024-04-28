use std::cell::RefCell;

use crate::{
    context::Context,
    event_station::{EventStation, Subscription},
    scope::ScopeKey,
};

pub struct EventListener<'scope, E: Clone + 'static, L: Listener<E> + Clone> {
    scope: ScopeKey,
    listener: L,
    subscriptions: Vec<(&'scope RefCell<EventStation<E>>, Subscription)>,
}

impl<'scope, E: Clone + 'static, L: Listener<E> + Clone> EventListener<'scope, E, L> {
    pub fn new(
        cx: &mut Context<'scope, '_>,
        listener: L,
        stations: &'scope [RefCell<EventStation<E>>],
    ) -> &'scope Self {
        let subscriptions = stations
            .into_iter()
            .map(|station| {
                // TODO: pls figure out how we can remove the whole RefCell trouble around events :<
                let sub = station
                    .borrow_mut()
                    .subscribe(cx.scope, listener.cloned_box());
                (station, sub)
            })
            .collect::<Vec<(&'scope RefCell<EventStation<E>>, Subscription)>>();
        let me = EventListener {
            scope: cx.scope,
            listener,
            subscriptions,
        };
        let whoops = cx.store(me);
        todo!()
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
