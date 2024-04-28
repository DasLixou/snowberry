use snowberry_core::{context::Context, event_station::EventStation};
use winit::{event::WindowEvent, window::Window};

use crate::{EventLoopContext, Windows};

// TODO: let the inner stuff run before the window so title can be set from inside, then add an oneshot builder after window creation
pub fn window<'scope: 'sub, 'sub>(
    cx: &'sub mut Context<'scope, '_>,
    title: &'static str,
    element: impl Fn(&mut Context<'sub, '_>, &'sub Window),
) {
    cx.sub_scope(|cx: &mut Context<'sub, '_>| {
        let Some(elc) = cx.resources.get_mut::<EventLoopContext>() else {
            eprintln!("Can't get EventLoopContext!");
            return;
        };

        let window = elc
            .active
            .create_window(Window::default_attributes().with_title(title))
            .unwrap();
        let id = window.id();
        let window = cx.store(window);

        let mut station = EventStation::new();
        let s = cx.scope; // currently we have to move it out of cx because the listener has to be 'static for now..
        station.listen(s, move |event, cx: &mut Context<'_, '_>| match event {
            WindowEvent::CloseRequested => {
                cx.close_scope(s);
            }
            _ => {}
        });

        let Some(windows) = cx.resources.get_mut::<Windows>() else {
            eprintln!("Can't get Windows resource!");
            return;
        };
        // TODO: make this insertion "recoverable" - some undo trait maybe?
        windows.event_handler.insert(id, station);

        (element)(cx, window);
    });
}
