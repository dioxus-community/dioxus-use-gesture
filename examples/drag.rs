use dioxus::prelude::*;
use dioxus_signals::use_signal;
use dioxus_spring::{use_animated, use_spring_signal};
use dioxus_use_gesture::{use_drag, DragState};
use std::time::Duration;

fn app(cx: Scope) -> Element {
    let element_ref = use_signal(cx, || None);

    let (spring_ref, value_ref) = use_spring_signal(cx, [0f32, 0f32]);
    use_animated(cx, element_ref, value_ref, |[x, y]| {
        format!("width: 200px; height: 200px; background: red; transform: translate({x}px, {y}px);")
    });

    use_drag(cx, element_ref, move |state, x, y| match state {
        DragState::Move => spring_ref.set([x, y]),
        DragState::End => spring_ref.animate([0., 0.], Duration::from_millis(500)),
    });

    log::info!("render");

    render!(div {
        onmounted: move |event| element_ref.set(Some(event.data.clone()))
    })
}

fn main() {
    dioxus_logger::init(log::LevelFilter::Info).expect("failed to init logger");

    dioxus_web::launch(app)
}
