#[macro_export]
macro_rules! run_simple {
    ($($tokens:tt)*) => {{
        fn hack<'a>(scope: Scope<'a>) {
            Composition::<'a, _>::root(scope, $($tokens)*, ());
        }
        $crate::scope::Scope::new(hack);
    }};
}
