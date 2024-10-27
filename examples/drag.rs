use dioxus::prelude::*;
use dioxus_spring::{use_animated, use_spring_signal};
use dioxus_use_gesture::{use_drag, DragState};
use dioxus_use_mounted::use_mounted;
use std::time::Duration;

fn app() -> Element {
    let element_ref = use_mounted();

    let (value_ref, spring_ref) = use_spring_signal([0f32, 0f32]);
    use_animated(element_ref, value_ref, |[x, y]| {
        format!("width: 200px; height: 200px; background: red; transform: translate({x}px, {y}px);")
    });

    use_drag(element_ref, move |state, x, y| match state {
        DragState::Move => spring_ref.set([x, y]),
        DragState::End => spring_ref.animate([0., 0.], Duration::from_millis(500)),
    });

    rsx!(div {
        onmounted: move |event| element_ref.onmounted(event)
    })
}

fn main() {
    dioxus::launch(app)
}
