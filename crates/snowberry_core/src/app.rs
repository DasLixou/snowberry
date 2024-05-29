use std::error::Error;

use crate::{element::InitElement, runner::Runner};

pub struct App {}

impl App {
    pub fn new() -> Self {
        App {}
    }

    pub fn run(
        self,
        mut runner: impl Runner,
        root: impl InitElement + 'static,
    ) -> Result<(), Box<dyn Error>> {
        runner.run(self, root)
    }
}
