# About
A simple graphics demo I made for learning Rust and to see how it suits game development. [wgpu](https://github.com/gfx-rs/wgpu) seemed like a good choice
for a low-level graphics API. There is no attempt to create an "engine", everything is pretty low level
(so as WGPU) and abstractions are being built along the way when needed.

![Screenshot](/screenshot.png?raw=true)

## Features
- Rendering via [wgpu](https://github.com/gfx-rs/wgpu).
- App structure via [Bevy ECS](https://crates.io/crates/bevy_ecs).
- Math via [nalgebra](https://github.com/dimforge/nalgebra).
- First person flying camera ("spectator") with protection from overturning.
- Physics via [Rapier](https://rapier.rs)
  - Simple rigid bodies with colliders.
  - Prevention of camera passing through objects (via character controller).
  - Ray casting.
- Skybox rendering on a full-screen quad.
- Post-processing: the scene is first rendered into a texture, which is then rendered on a full-screen quad
with a separate shader (currently applying vignette).
- Debug UI via [ImGui](https://github.com/yatekii/imgui-wgpu-rs).


## Building and running
```
cargo run
```
See on-screen instructions for controls.
