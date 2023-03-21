# About
A simple graphics demo I made for learning Rust and testing how it suits game development. WGPU seemed like a good choice
for a low-level graphics API. There is no attempt to create an "engine", everything is pretty low level
(so as WGPU) and abstractions are being build along the way when needed.

![Screenshot](/screenshot.png?raw=true)

## Building and running
```
cargo run
```
Right mouse click to control camera, use `W-A-S-D-Q-E` keys to fly around.

## Features
- Math based on [nalgebra](https://github.com/dimforge/nalgebra).
- Physics via [Rapier](https://rapier.rs)
  - Simple rigid bodies with colliders.
  - Character controller to prevent camera from going through objects.
  - Ray casting.
- Skybox rendering on a full-screen quad.
- First person flying camera ("spectator") with protection from overturning.
- Cursor capturing when controlling the camera.
- Render to texture: the scene is first rendered into a low-res texture, which is then rendered on a full-screen quad to achieve pixelated effect.
- Debug UI via [ImGui](https://github.com/yatekii/imgui-wgpu-rs).
- Rendering scene objects via WGPU's `RenderBundle` instead of directly via `RenderPass`. This proved to be _very_ useful
if you wish to decouple scene rendering logic from render passes and make it modular. With render passes I couldn't
make it work due to Rust borrow checker and different object lifetimes involved - as in many examples I found,
the whole rendering code had to be within one function/block in order for lifetimes of render passes, textures and other
things to work together.
