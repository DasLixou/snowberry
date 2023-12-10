mod branch;

use std::marker::PhantomData;

pub use branch::Branch;

pub struct DescConstruct<F: FnOnce(Inputs, Params), Inputs, Params> {
    phantom: PhantomData<Params>,
    inputs: Inputs,
    params: Params, // TODO: make them auto generated
    f: F,
}

pub trait Constructable<Inputs, Params>: FnOnce(Inputs, Params) + Sized {
    fn construct(self, inputs: Inputs) -> DescConstruct<Self, Inputs, Params>;
}

impl<F, Inputs> Constructable<Inputs, ()> for F
where
    F: FnOnce(Inputs, ()),
{
    fn construct(self, inputs: Inputs) -> DescConstruct<Self, Inputs, ()> {
        DescConstruct {
            phantom: PhantomData,
            inputs,
            params: (),
            f: self,
        }
    }
}

impl<F, Inputs> Constructable<Inputs, Branch> for F
where
    F: FnOnce(Inputs, Branch),
{
    fn construct(self, inputs: Inputs) -> DescConstruct<Self, Inputs, Branch> {
        DescConstruct {
            phantom: PhantomData,
            inputs,
            params: Branch {},
            f: self,
        }
    }
}

pub trait Construct {
    fn build(self);
}

impl<F: FnOnce(Inputs, Params), Inputs, Params> Construct for DescConstruct<F, Inputs, Params> {
    fn build(self) {
        (self.f)(self.inputs, self.params)
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
