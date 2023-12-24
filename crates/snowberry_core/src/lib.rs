mod branch;
mod construct_param;
mod tree;

use std::marker::PhantomData;

pub use branch::Branch;
use construct_param::ConstructParam;
use tree::{BranchIdx, Tree};

pub struct DescConstruct<C: Constructable<Inputs, Params>, Inputs, Params: ConstructParam> {
    phantom: PhantomData<Params>,
    inputs: Inputs,
    constructable: C,
}

pub trait Constructable<Inputs, Params: ConstructParam>:
    FnOnce(Inputs, Params) + FnOnce(Inputs, Params::Param<'_>) + Sized
{
    fn construct(self, inputs: Inputs) -> DescConstruct<Self, Inputs, Params>;
}

impl<F, Inputs, P1: ConstructParam> Constructable<Inputs, P1> for F
where
    F: FnOnce(Inputs, P1) + FnOnce(Inputs, P1::Param<'_>),
{
    fn construct(self, inputs: Inputs) -> DescConstruct<Self, Inputs, P1> {
        DescConstruct {
            phantom: PhantomData,
            inputs,
            constructable: self,
        }
    }
}

pub trait Construct {
    fn build(self, tree: &Tree, idx: BranchIdx);
}

impl<C: Constructable<Inputs, Params>, Inputs, Params: ConstructParam> Construct
    for DescConstruct<C, Inputs, Params>
{
    fn build(self, tree: &Tree, idx: BranchIdx) {
        let params = Params::bake_param(tree, idx);
        (self.constructable)(self.inputs, params);
    }
}

pub struct Snowberry {
    tree: Tree,
}

impl Snowberry {
    pub fn new() -> Self {
        Self { tree: Tree::new() }
    }

    pub fn add_root(&mut self, construct: impl Construct) -> &mut Self {
        let idx = self.tree.new_entry(); // TODO: can we make this with &mut self pls?
        construct.build(&self.tree, idx);
        self
    }
}
