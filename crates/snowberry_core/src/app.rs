use std::error::Error;

use crate::{element::Element, runner::Runner};

pub struct App {}

impl App {
    pub fn new() -> Self {
        App {}
    }

    pub fn run<'root>(
        self,
        mut runner: impl Runner,
        root: impl Element<'root> + 'static,
    ) -> Result<(), Box<dyn Error>> {
        runner.run(self, root)
    }
}
