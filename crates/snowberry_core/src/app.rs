use std::error::Error;

use crate::{element::Element, runner::Runner};

pub struct App {}

impl App {
    pub fn new() -> Self {
        App {}
    }

    pub fn run(
        self,
        mut runner: impl Runner,
        root: impl Element + 'static,
    ) -> Result<(), Box<dyn Error>> {
        runner.run(self, Box::new(root))
    }
}
