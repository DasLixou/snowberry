use slotmap::{new_key_type, SlotMap};

new_key_type! { pub struct BranchIdx; }

pub struct BranchInfo {
    pub(crate) children: Vec<BranchIdx>,
}

pub struct Tree {
    entries: SlotMap<BranchIdx, BranchInfo>,
}

impl Tree {
    pub fn new() -> Self {
        Self {
            entries: SlotMap::with_key(),
        }
    }

    pub fn insert(&mut self, info: BranchInfo) -> BranchIdx {
        self.entries.insert(info)
    }
}
