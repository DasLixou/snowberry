#[macro_export]
macro_rules! run_simple {
    ($($tokens:tt)*) => {
        $crate::composition::Composition::root($($tokens)*);
    };
}
