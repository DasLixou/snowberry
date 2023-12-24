use crate::tree::{BranchIdx, Tree};

pub trait ConstructParam {
    type Param<'s>: ConstructParam;

    fn bake_param<'s>(tree: &'s Tree, idx: BranchIdx) -> Self::Param<'s>;
}

impl ConstructParam for () {
    type Param<'s> = ();

    fn bake_param<'s>(_tree: &'s Tree, _idx: BranchIdx) -> Self::Param<'s> {
        ()
    }
}
