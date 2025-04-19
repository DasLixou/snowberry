#[macro_export]
macro_rules! event {
    ($cx:expr) => {{
        struct AnonymousEvent<'cx>($crate::InvariantLifetime<'cx>);
        unsafe impl<'cx> $crate::event::Event for AnonymousEvent<'cx> {}

        fn life_event<'cx, Env>(
            cx: $crate::context::Context<'cx, Env>,
        ) -> ($crate::context::Context<'cx, Env>, AnonymousEvent<'cx>) {
            (cx, AnonymousEvent(::core::marker::PhantomData))
        }
        println!("Event: {:?}", ::core::any::TypeId::of::<AnonymousEvent>());
        life_event($cx)
    }};
}

pub unsafe trait Event {}
