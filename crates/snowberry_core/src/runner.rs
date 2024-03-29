use std::error::Error;

use crate::{app::App, scope::Scope};

pub trait Runner {
    type Data;

    fn run(&mut self, app: App, root: Box<dyn Scope<Self>>) -> Result<(), Box<dyn Error>>;
}
