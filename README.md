# dioxus-use-gesture

```rust
let spring_ref = use_spring_style(cx, [0f32, 0f32], |[x, y]| {
    format!("width: 200px; height: 200px; background: red; transform: translate({x}px, {y}px);")
});

let spring_ref_clone = spring_ref.clone();
let drag_ref = use_drag(cx, move |state, x, y| match state {
    DragState::Move => spring_ref_clone.set([x, y]),
    DragState::End => spring_ref_clone.animate([0., 0.], Duration::from_millis(500)),
});

render!(div {
    onmounted: move |event| {
        spring_ref.mount(event.data.clone());
        drag_ref.mount(event.data);
    }
})
```
