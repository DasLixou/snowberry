use snowberry_core::{context::Context, element::Element, event_station::EventStation};
use winit::{event::WindowEvent, window::WindowBuilder};

use crate::{EventLoopContext, Windows};

pub fn window<'scope>(cx: &mut Context<'scope, '_>, title: &'static str, _scope: impl Element) {
    cx.sub_scope(|cx: &mut Context<'_, '_>| {
        let Some(elc) = cx.resources.get_mut::<EventLoopContext>() else {
            eprintln!("Can't get EventLoopContext!");
            return;
        };

        let window = WindowBuilder::new()
            .with_title(title)
            .build(elc.window_target)
            .unwrap();
        let id = window.id();
        cx.store(window);

        let mut station = EventStation::new();
        let s = cx.scope; // currently we have to move it out of cx because the listener has to be 'static for now..
        station.listen(
            s,
            move |event: &WindowEvent, _cx: &mut Context<'_, '_>| match event {
                WindowEvent::CloseRequested => {
                    println!("TODO: close me :3 (and kill scope {:?})", s)
                }
                _ => {}
            },
        );

        let Some(windows) = cx.resources.get_mut::<Windows>() else {
            eprintln!("Can't get Windows resource!");
            return;
        };
        // TODO: make this insertion "recoverable" - some undo trait maybe?
        windows.event_handler.insert(id, station);
    });
}
