# About

A simple graphics demo I made to learn Rust and try it in game development.

There is no attempt to create an "engine", everything is pretty low level and abstractions are built
along the way when needed.

![Demo](/demo.gif?raw=true)

## Building and running

```
cargo run
```

## Features

- [wgpu](https://github.com/gfx-rs/wgpu) rendering.
- [Dear ImGui](https://github.com/yatekii/imgui-wgpu-rs) UI.
- [nalgebra](https://github.com/dimforge/nalgebra) math.
- [Rapier](https://rapier.rs) physics
    - Rigid bodies with colliders.
    - Camera with character controller, preventing it from passing through objects.
    - Ray casting.
    - Drag-n-drop.
- First person flying camera ("spectator") with clamping of vertical angles to protect from overturning.
- Skybox rendering on a full-screen quad.
- Vignette post-processing.
