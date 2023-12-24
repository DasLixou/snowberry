use crate::{
    construct_param::ConstructParam,
    tree::{BranchIdx, Tree},
    Construct,
};

pub struct Branch<'t> {
    tree: &'t Tree,
    idx: BranchIdx,
}

impl<'t> Branch<'t> {
    pub fn add_child(&mut self, child: impl Construct) {
        let sub_idx = self.tree.new_entry();
        child.build(self.tree, sub_idx);
        self.tree.add_child(self.idx, sub_idx);
    }
}

impl ConstructParam for Branch<'_> {
    type Param<'s> = Branch<'s>;

    fn bake_param<'s>(tree: &'s Tree, idx: BranchIdx) -> Self::Param<'s> {
        Branch { tree, idx }
    }
}
