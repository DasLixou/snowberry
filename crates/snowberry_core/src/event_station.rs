use slotmap::{new_key_type, SlotMap};

use crate::{context::Context, event_listener::Listener, scope::ScopeKey};

pub trait EventDispatcher {
    fn dispatch(&self, erased_station: ErasedEventStation);
}

new_key_type! {
    pub struct Subscription;
}

pub struct EventStation<E: Clone + 'static> {
    pub listeners: SlotMap<Subscription, (ScopeKey, Box<dyn Listener<E>>)>,
}

impl<E: Clone + 'static> EventStation<E> {
    pub fn new() -> Self {
        Self {
            listeners: SlotMap::with_key(),
        }
    }

    pub(crate) fn subscribe(&mut self, scope: ScopeKey, l: Box<dyn Listener<E>>) -> Subscription {
        self.listeners.insert((scope, l))
    }

    pub(crate) fn unsubscribe(&mut self, subscription: Subscription) {
        self.listeners.remove(subscription);
    }

    pub fn dispatch(&self, cx: &mut Context<'_, '_>, event: E) {
        cx.event_dispatcher.dispatch(self.to_erased(event));
    }

    fn to_erased(&self, event: E) -> ErasedEventStation {
        ErasedEventStation {
            listener_calls: self
                .listeners
                .values()
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
