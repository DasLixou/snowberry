use std::error::Error;

use snowberry::core::{App, Context};
use snowberry::winit::WinitRunner;

fn main() -> Result<(), Box<dyn Error>> {
    App::new().run(WinitRunner, content)
}

struct Content {}

fn content(cx: &mut Context) -> Content {
    // IDEA: one shot event / async wait for window creation
    Content {}
}
