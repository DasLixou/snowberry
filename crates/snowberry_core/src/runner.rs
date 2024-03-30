use std::error::Error;

use crate::{app::App, element::Element};

pub trait Runner {
    type Data;

    fn run(&mut self, app: App, root: Box<dyn Element<Self>>) -> Result<(), Box<dyn Error>>;
}
