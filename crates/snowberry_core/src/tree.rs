use std::cell::RefCell;

use slotmap::{new_key_type, SecondaryMap, SlotMap};

new_key_type! { pub struct BranchIdx; }

pub struct Tree {
    entries: RefCell<SlotMap<BranchIdx, ()>>,
    children: RefCell<SecondaryMap<BranchIdx, Vec<BranchIdx>>>,
}

impl Tree {
    pub fn new() -> Self {
        Self {
            entries: RefCell::new(SlotMap::with_key()),
            children: RefCell::new(SecondaryMap::new()),
        }
    }

    pub fn new_entry(&self) -> BranchIdx {
        self.entries.borrow_mut().insert(())
    }

    pub fn add_child(&self, parent: BranchIdx, child: BranchIdx) {
        self.children
            .borrow_mut()
            .entry(parent)
            .unwrap()
            .or_default()
            .push(child);
    }
}
