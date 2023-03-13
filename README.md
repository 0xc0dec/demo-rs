# About
A simple graphics demo I made for learning Rust and testing its applicability to gamedev. WGPU seemed like a good choice
for a low-level graphics API. There is no attempt to create an "engine", everything is pretty low level
(so as WGPU) and abstractions are being build along the way only when needed.

What's implemented so far:
- Rendering scene objects via WGPU's `RenderBundle` instead of directly via `RenderPass`. This proved to be _very_ useful
if you wish to decouple scene rendering logic from render passes and make it more modular. With render passes I couldn't
make it work due to the Rust borrow checker and different object lifetimes involved.
- Skybox rendering on a full-screen quad.
- First person camera flying camera ("spectator") with protection from overturning.
- Render to texture.
- Physics via Rapier3D
  - Simple rigid bodies with colliders.
  - Character controller to prevent camera from going through objects.
  - Ray casting.

![Screenshot](/screenshot.png?raw=true)