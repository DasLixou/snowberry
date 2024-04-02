use std::error::Error;

use crate::{app::App, element::Element};

pub trait Runner {
    fn run(&mut self, app: App, root: Box<dyn Element>) -> Result<(), Box<dyn Error>>;
}
