use std::error::Error;

use crate::{runner::Runner, scope::Scope};

pub struct App {}

impl App {
    pub fn new() -> Self {
        App {}
    }

    pub fn run<R: Runner>(
        self,
        mut runner: R,
        root: impl Scope<R> + 'static,
    ) -> Result<(), Box<dyn Error>> {
        runner.run(self, Box::new(root))
    }
}
