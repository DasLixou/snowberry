mod build_cx;
pub use build_cx::BuildContext;

pub struct Snowberry {}

impl Snowberry {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run<F>(&mut self, root: F)
    where
        F: FnOnce(&BuildContext),
    {
        root(&BuildContext {})
    }
}
