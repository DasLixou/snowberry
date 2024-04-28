use std::error::Error;

use crate::{app::App, element::Element};

pub trait Runner {
    fn run<'root>(&mut self, app: App, root: impl Element<'root>) -> Result<(), Box<dyn Error>>;
}
