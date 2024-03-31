use snowberry_core::{context::Context, element::Element};

use crate::{WinitRunner, WinitRunnerEvent};

pub fn window<'scope>(
    cx: Context<'scope, WinitRunner>,
    title: &'static str,
    _scope: impl Element<WinitRunner>,
) {
    cx.runner_data
        .send_event(WinitRunnerEvent::CreateWindow(title.to_string()))
        .unwrap();
}
