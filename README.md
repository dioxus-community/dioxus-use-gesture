<div align="center">
<h1>Dioxus use_gesture</h1>
 <a href="https://crates.io/crates/dioxus-use-gesture">
    <img src="https://img.shields.io/crates/v/dioxus-use-gesture?style=flat-square"
    alt="Crates.io version" />
  </a>
  <a href="https://docs.rs/dioxus-use-gesture/latest/dioxus-use-gesture">
    <img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square"
      alt="docs.rs docs" />
  </a>
   <a href="https://github.com/matthunz/dioxus-use-gesture/actions">
    <img src="https://github.com/matthunz/dioxus-use-gesture/actions/workflows/ci.yml/badge.svg"
      alt="CI status" />
  </a>
</div>

<div align="center">
 <a href="https://github.com/matthunz/dioxus-use-gesture/tree/main/examples">Examples</a>
</div>

<br>

```rust
let spring_ref = use_spring_style(cx, [0f32, 0f32], |[x, y]| {
    format!("width: 200px; height: 200px; background: red; transform: translate({x}px, {y}px);")
});

let drag_ref = use_drag(cx, move |state, x, y| match state {
    DragState::Move => spring_ref.set([x, y]),
    DragState::End => spring_ref.animate([0., 0.], Duration::from_millis(500)),
});

render!(div {
    onmounted: move |event| {
        spring_ref.mount(event.data.clone());
        drag_ref.mount(event.data);
    }
})
```
