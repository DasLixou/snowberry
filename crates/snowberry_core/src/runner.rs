use std::error::Error;

use crate::{app::App, element::InitElement};

pub trait Runner {
    fn run(&mut self, app: App, root: impl InitElement) -> Result<(), Box<dyn Error>>;
}
