# About

A simple graphics demo I made to learn Rust and try it in game development.

There is no attempt to create an "engine", everything is pretty low level and abstractions are being built
along the way when needed.

![Screenshot](/screenshot.png?raw=true)

## Building and running

```
cargo run
```

## Features

- [Wgpu](https://github.com/gfx-rs/wgpu) rendering.
- [ImGui](https://github.com/yatekii/imgui-wgpu-rs) UI.
- [Bevy ECS](https://crates.io/crates/bevy_ecs).
- [nalgebra](https://github.com/dimforge/nalgebra) math.
- [Rapier](https://rapier.rs) physics
    - Rigid bodies with colliders.
    - Camera with character controller, preventing it from passing through objects.
    - Ray casting.
    - Drag-n-drop.
- First person flying camera ("spectator") with protection against overturning.
- Skybox rendering on a full-screen quad.
- Vignette post-processing effect.
