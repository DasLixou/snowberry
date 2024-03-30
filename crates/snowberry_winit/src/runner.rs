use std::error::Error;

use snowberry_core::{app::App, context::Context, element::Element, runner::Runner};
use winit::{
    event::{Event, StartCause, WindowEvent},
    event_loop::{EventLoopBuilder, EventLoopProxy},
    window::WindowBuilder,
};

#[derive(Debug)]
pub enum WinitRunnerEvent {
    CreateWindow(String),
}

pub struct WinitRunner;

impl Runner for WinitRunner {
    type Data = EventLoopProxy<WinitRunnerEvent>;

    fn run(&mut self, _app: App, root: Box<dyn Element<Self>>) -> Result<(), Box<dyn Error>> {
        let event_loop = EventLoopBuilder::<WinitRunnerEvent>::with_user_event().build()?;
        let proxy = event_loop.create_proxy();

        event_loop.run(move |event, elwt| {
            //println!("{event:?}");
            match event {
                Event::NewEvents(StartCause::Init) => {
                    root.build(Context {
                        runner_data: proxy.clone(),
                    });
                    println!("Root was built!");
                }
                Event::WindowEvent { event, .. } => match event {
                    // TODO: send that to the window
                    WindowEvent::CloseRequested => {
                        elwt.exit();
                    }
                    _ => {}
                },
                Event::UserEvent(winit_runner_event) => match winit_runner_event {
                    WinitRunnerEvent::CreateWindow(title) => {
                        let window = WindowBuilder::new().with_title(title).build(&elwt).unwrap();
                        // TODO: give that back to the window scope
                        std::mem::forget(window);
                    }
                },
                _ => {}
            }
        })?;

        Ok(())
    }
}
