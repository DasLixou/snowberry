use std::error::Error;

use crate::{element::Element, runner::Runner};

pub struct App {}

impl App {
    pub fn new() -> Self {
        App {}
    }

    pub fn run<R: Runner>(
        self,
        mut runner: R,
        root: impl Element<R> + 'static,
    ) -> Result<(), Box<dyn Error>> {
        runner.run(self, Box::new(root))
    }
}
