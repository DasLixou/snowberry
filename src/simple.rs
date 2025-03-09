#[macro_export]
macro_rules! run_simple {
    ($($tokens:tt)*) => {
        $crate::composition::Composition::open($($tokens)*);
    };
}
