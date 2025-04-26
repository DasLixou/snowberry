use std::marker::PhantomData;

use imply_hack::Imply;

use crate::environment::WithEnv;

#[macro_export]
macro_rules! event {
    ($cx:expr) => {{
        struct AnonymousEvent<'cx>($crate::InvariantLifetime<'cx>);
        unsafe impl<'cx, Env> $crate::event_::Event<Env> for AnonymousEvent<'cx> where
            Env: $crate::environment::WithEnv<$crate::event_::Callchain<Self>>
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

pub struct Callchain<Event> {
    pub phantom: PhantomData<Event>,
}

pub unsafe trait Event<Env>
where
    Self: Sized,
    Self: Imply<Env, Is: WithEnv<Callchain<Self>>>,
{
}
