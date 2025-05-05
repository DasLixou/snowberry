use std::marker::PhantomData;

use crate::{context::Context, event_env::WithCallback};
use imply_hack::Imply;

#[macro_export]
macro_rules! event {
    ($cx:expr) => {{
        struct AnonymousEvent<'cx>($crate::InvariantLifetime<'cx>);
        unsafe impl<'cx, Env> $crate::event_::Event<Env> for AnonymousEvent<'cx> where
            Env: $crate::event_env::WithCallback<$crate::event_::Callchain<Self>>
        {
        }

        fn life_event<'cx, Env>(
            cx: $crate::context::Context<'cx, Env>,
        ) -> ($crate::context::Context<'cx, Env>, AnonymousEvent<'cx>) {
            (cx, AnonymousEvent(::core::marker::PhantomData))
        }
        println!("Event: {:?}", ::core::any::TypeId::of::<AnonymousEvent>());
        life_event($cx)
    }};
}

pub unsafe trait Event<Env>
where
    Self: Sized,
    // Remove the Callback<> wrapper, it's just for now so I can name it easier
    Self: Imply<Env, Is: WithCallback<Callchain<Self>>>,
{
    fn dispatch(&self, cx: &Context<'_, Env>) {
        let x = cx.env.get();
        (x)()
    }
}

pub struct Callchain<Event> {
    pub phantom: PhantomData<Event>,
}
impl<Event> Clone for Callchain<Event> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<Event> Copy for Callchain<Event> {}

// impl<Event, F: Copy + Fn()> Environmentable for (Callchain<Event>, F) {}
