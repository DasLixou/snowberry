pub mod dynamic;
pub mod event;
mod event_map;
mod runner;

use event_map::EventMap;
pub use runner::*;
mod build_cx;
pub use build_cx::Context;
use snowberry_arena::DropArena;
use type_map::TypeMap;

pub struct Snowberry {
    pub global_resources: TypeMap,
    pub root_bank: DropArena,
    pub cleanup: Vec<Box<dyn FnOnce(&mut EventMap)>>,
    pub events: EventMap,
}

impl Snowberry {
    pub fn new() -> Self {
        Self {
            global_resources: TypeMap::new(),
            root_bank: DropArena::new(),
            cleanup: Vec::new(),
            events: EventMap::new(),
        }
    }

    pub fn run<F>(mut self, runner: impl Runner, root: F)
    where
        F: FnOnce(Context),
    {
        root(Context {
            global_resources: &mut self.global_resources,
            bank: &self.root_bank,
            cleanup: &mut self.cleanup,
            events: &mut self.events,
        });

        runner.run(self);
    }
}
