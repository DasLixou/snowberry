pub trait Construct {
    fn build(self);
}

impl<F> Construct for F
where
    F: FnOnce(),
{
    fn build(self) {
        self()
    }
}

pub struct Snowberry {}

impl Snowberry {
    pub fn new() -> Self {
        Self {}
    }

    pub fn add_root(&mut self, construct: impl Construct) -> &mut Self {
        construct.build();
        self
    }
}
