use dioxus::prelude::*;
use dioxus_signals::{use_signal, Signal};
use std::rc::Rc;
use wasm_bindgen::{prelude::Closure, JsCast};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DragState {
    Move,
    End,
}

#[derive(Default)]
struct State {
    on_pointer_down: Option<Closure<dyn FnMut(web_sys::PointerEvent)>>,
    on_pointer_move: Option<Closure<dyn FnMut(web_sys::PointerEvent)>>,
    on_pointer_up: Option<Closure<dyn FnMut(web_sys::PointerEvent)>>,
    start: Option<(f32, f32)>,
}

pub fn use_drag<T>(
    cx: Scope<T>,
    element_ref: Signal<Option<Rc<MountedData>>>,
    on_drag: impl FnMut(DragState, f32, f32) + 'static,
) {
    let state_ref = use_signal(cx, || State::default());
    let handler_cell = Rc::new(RefCell::new(on_drag));

    use_effect(cx, &element_ref.read().is_some(), move |_| async move {
        let mut state = state_ref.write();
        if let Some(mounted) = element_ref.read().clone() {
            let element = mounted
                .get_raw_element()
                .unwrap()
                .downcast_ref::<web_sys::Element>()
                .unwrap();

            let handler_cell_clone = handler_cell.clone();
            state.on_pointer_move = Some(Closure::new(move |event: web_sys::PointerEvent| {
                if let Some((start_x, start_y)) = state_ref.read().start {
                    handler_cell_clone.borrow_mut()(
                        DragState::Move,
                        event.client_x() as f32 - start_x,
                        event.client_y() as f32 - start_y,
                    );
                }
            }));
            add_listener(element, "pointermove", &state.on_pointer_move);

            let callback_mounted = mounted.clone();
            state.on_pointer_down = Some(Closure::new(move |event: web_sys::PointerEvent| {
                let element = callback_mounted
                    .get_raw_element()
                    .unwrap()
                    .downcast_ref::<web_sys::Element>()
                    .unwrap();

                let rect = element.get_bounding_client_rect();

                state_ref.write().start = Some((
                    event.client_x() as f32 - rect.left() as f32,
                    event.client_y() as f32 - rect.top() as f32,
                ));
            }));
            add_listener(element, "pointerdown", &state.on_pointer_down);

            state.on_pointer_up = Some(Closure::new(move |event: web_sys::PointerEvent| {
                handler_cell.borrow_mut()(
                    DragState::End,
                    event.client_x() as _,
                    event.client_y() as _,
                );
                state_ref.write().start.take();
            }));
            add_listener(element, "pointerup", &state.on_pointer_up);
        }
    });

    let state_ref_clone = state_ref.clone();
    use_on_destroy(cx, move || {
        let mut state = state_ref_clone.write();
        if let Some(mounted) = element_ref.read().clone() {
            let element = mounted
                .get_raw_element()
                .unwrap()
                .downcast_ref::<web_sys::Element>()
                .unwrap();

            remove_listener(element, "pointerdown", &mut state.on_pointer_down);
            remove_listener(element, "pointermove", &mut state.on_pointer_move);
            remove_listener(element, "pointerup", &mut state.on_pointer_up);
        }
    });
}

fn add_listener(
    element: &web_sys::Element,
    name: &str,
    cell: &Option<Closure<dyn FnMut(web_sys::PointerEvent)>>,
) {
    element
        .add_event_listener_with_callback(name, cell.as_ref().unwrap().as_ref().unchecked_ref())
        .unwrap();
}

fn remove_listener(
    element: &web_sys::Element,
    name: &str,
    cell: &mut Option<Closure<dyn FnMut(web_sys::PointerEvent)>>,
) {
    if let Some(f) = cell.take() {
        element
            .remove_event_listener_with_callback(name, f.as_ref().unchecked_ref())
            .unwrap();
    }
}
