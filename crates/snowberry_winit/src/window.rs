use snowberry_core::{context::Context, scope::Scope};

use crate::{WinitRunner, WinitRunnerEvent};

pub fn window(cx: Context<WinitRunner>, title: &'static str, _scope: impl Scope<WinitRunner>) {
    cx.runner_data
        .send_event(WinitRunnerEvent::CreateWindow(title.to_string()))
        .unwrap();
}
