mod branch;

use std::marker::PhantomData;

pub use branch::Branch;

pub trait ConstructFn<Params>: Sized {
    fn build(self);
    fn construct(self) -> DescConstruct<Self, Params>;
}

pub struct DescConstruct<F, Params>(F, PhantomData<Params>);

pub trait Construct {
    fn build(self);
}

impl<F> ConstructFn<()> for F
where
    F: FnOnce(),
{
    fn build(self) {
        self()
    }

    fn construct(self) -> DescConstruct<Self, ()> {
        DescConstruct(self, PhantomData)
    }
}

impl<F> ConstructFn<(Branch,)> for F
where
    F: FnOnce(Branch),
{
    fn build(self) {
        self(Branch {})
    }

    fn construct(self) -> DescConstruct<Self, (Branch,)> {
        DescConstruct(self, PhantomData)
    }
}

impl<Params, F: ConstructFn<Params>> Construct for DescConstruct<F, Params> {
    fn build(self) {
        self.0.build()
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
