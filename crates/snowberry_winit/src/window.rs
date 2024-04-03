use snowberry_core::{context::Context, element::Element, event_station::EventStation};
use winit::{event::WindowEvent, window::WindowBuilder};

use crate::{EventLoopContext, Windows};

pub fn window<'scope>(cx: &mut Context<'scope, '_>, title: &'static str, _scope: impl Element) {
    let Some(elc) = cx.resources.get_mut::<EventLoopContext>() else {
        eprintln!("Can't get EventLoopContext!");
        return;
    };

    let window = WindowBuilder::new()
        .with_title(title)
        .build(elc.window_target)
        .unwrap();
    let id = window.id();
    cx.scope.store(window);

    let mut station = EventStation::new();
    station.listen(|event| match event {
        WindowEvent::CloseRequested => {
            println!("close me :3")
        }
        _ => {}
    });

    let Some(windows) = cx.resources.get_mut::<Windows>() else {
        eprintln!("Can't get Windows resource!");
        return;
    };
    windows.event_handler.insert(id, station);
}
