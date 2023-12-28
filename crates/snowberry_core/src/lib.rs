pub mod dynamic;
pub mod event;
mod runner;

use bumpalo::Bump;
pub use runner::*;
mod build_cx;
pub use build_cx::Context;
use type_map::TypeMap;

pub struct Snowberry {
    pub global_resources: TypeMap,
    pub root_bank: Bump,
}

impl Snowberry {
    pub fn new() -> Self {
        Self {
            global_resources: TypeMap::new(),
            root_bank: Bump::new(),
        }
    }

    pub fn run<F>(mut self, runner: impl Runner, root: F)
    where
        F: FnOnce(Context),
    {
        root(Context {
            global_resources: &mut self.global_resources,
            bank: &mut self.root_bank,
        });

        runner.run(self);
    }
}
