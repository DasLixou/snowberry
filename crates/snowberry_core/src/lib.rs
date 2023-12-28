mod runner;

pub use runner::*;
mod build_cx;
pub use build_cx::Context;
use type_map::TypeMap;

pub struct Snowberry {
    pub global_resources: TypeMap,
}

impl Snowberry {
    pub fn new() -> Self {
        Self {
            global_resources: TypeMap::new(),
        }
    }

    pub fn run<F>(mut self, runner: impl Runner, root: F)
    where
        F: FnOnce(Context),
    {
        root(Context {
            global_resources: &mut self.global_resources,
        });

        runner.run(self);
    }
}
