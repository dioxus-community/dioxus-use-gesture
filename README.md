<div align="center">
<h1>Dioxus use_gesture</h1>
 <a href="https://crates.io/crates/dioxus-use-gesture">
    <img src="https://img.shields.io/crates/v/dioxus-use-gesture?style=flat-square"
    alt="Crates.io version" />
  </a>
  <a href="https://docs.rs/dioxus-use-gesture/latest/dioxus_use_gesture/">
    <img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square"
      alt="docs.rs docs" />
  </a>
   <a href="https://github.com/dioxus-community/dioxus-use-gesture/actions">
    <img src="https://github.com/dioxus-community/dioxus-use-gesture/actions/workflows/ci.yml/badge.svg"
      alt="CI status" />
  </a>
</div>

<div align="center">
 <a href="https://github.com/dioxus-community/dioxus-use-gesture/tree/main/examples">Examples</a>
</div>

<br>

Gesture interaction library for [Dioxus](https://dioxuslabs.com).

Pairs great with [dioxus-spring](https://github.com/dioxus-community/dioxus-spring)!


```rust
let element_ref = use_signal(cx, || None);

let (spring_ref, value_ref) = use_spring_signal(cx, [0f32, 0f32]);
use_animated(cx, element_ref, value_ref, |[x, y]| {
    format!("width: 200px; height: 200px; background: red; transform: translate({x}px, {y}px);")
});

use_drag(cx, element_ref, move |state, x, y| match state {
    DragState::Move => spring_ref.set([x, y]),
    DragState::End => spring_ref.animate([0., 0.], Duration::from_millis(500)),
});

render!(div {
    onmounted: move |event| element_ref.set(Some(event.data.clone()))
})
```
