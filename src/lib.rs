use std::rc::Rc;
use dioxus::prelude::*;
use wasm_bindgen::{prelude::Closure, JsCast};

pub enum DragState {
    Move,
    End,
}

#[derive(Default)]
struct State {
    mounted: Option<Rc<MountedData>>,
    callbacks: Vec<Closure<dyn FnMut(web_sys::PointerEvent)>>,
    start: Option<(f32, f32)>,
}

pub fn use_drag<T>(cx: Scope<T>, f: impl FnMut(DragState, f32, f32) + 'static) -> UseDrag {
    let state_ref = use_ref(cx, || State::default());

    let state_ref_clone = state_ref.clone();

    let handler_cell = Rc::new(RefCell::new(f));
    use_effect(
        cx,
        &state_ref.read().mounted.is_some(),
        move |_| async move {
            let mut state = state_ref_clone.write();
            if let Some(mounted) = state.mounted.clone() {
                let element = mounted
                    .get_raw_element()
                    .unwrap()
                    .downcast_ref::<web_sys::Element>()
                    .unwrap();

                let callback_state_ref = state_ref_clone.clone();
                let handler_cell_clone = handler_cell.clone();
                state
                    .callbacks
                    .push(Closure::new(move |event: web_sys::PointerEvent| {
                        if let Some((start_x, start_y)) =callback_state_ref.read().start {
                            handler_cell_clone.borrow_mut()(
                                DragState::Move,
                                 event.client_x() as f32 - start_x,
                                event.client_y() as f32 - start_y,
                            );
                        }
                    }));
                element
                    .add_event_listener_with_callback(
                        "pointermove",
                        state.callbacks[0].as_ref().unchecked_ref(),
                    )
                    .unwrap();

                let callback_state_ref = state_ref_clone.clone();
                let callback_mounted = mounted.clone();
                state
                    .callbacks
                    .push(Closure::new(move |event: web_sys::PointerEvent| {
                        let element = callback_mounted
                            .get_raw_element()
                            .unwrap()
                            .downcast_ref::<web_sys::Element>()
                            .unwrap();
                        let rect = element.get_bounding_client_rect();

                        callback_state_ref.write().start = Some((
                            event.client_x() as f32 - rect.left() as f32,
                            event.client_y() as f32 - rect.top() as f32,
                        ));
                    }));
                element
                    .add_event_listener_with_callback(
                        "pointerdown",
                        state.callbacks[1].as_ref().unchecked_ref(),
                    )
                    .unwrap();

                let callback_state_ref = state_ref_clone.clone();
                state
                    .callbacks
                    .push(Closure::new(move |event: web_sys::PointerEvent| {
                        handler_cell.borrow_mut()(
                            DragState::End,
                            event.client_x() as _,
                            event.client_y() as _,
                        );
                        callback_state_ref.write().start.take();
                    }));
                element
                    .add_event_listener_with_callback(
                        "pointerup",
                        state.callbacks[2].as_ref().unchecked_ref(),
                    )
                    .unwrap();
            }
        },
    );

    UseDrag {
        element_ref: state_ref.clone(),
    }
}

pub struct UseDrag {
    element_ref: UseRef<State>,
}

impl UseDrag {
    pub fn mount(&self, data: Rc<MountedData>) {
        self.element_ref.write().mounted = Some(data);
    }
}
