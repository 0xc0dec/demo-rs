# About
A simple graphics demo I made for learning Rust and testing its applicability to gamedev. WGPU seemed like a good choice
for a low-level graphics API. There is no attempt to create an "engine", everything is pretty low level
(so as WGPU) and abstractions are being build along the way only when needed.

What's implemented so far:
- Skybox rendering on a full-screen quad.
- Render to texture.
- Physics via Rapier3D
  - Simple rigid bodies with colliders.
  - Character controller to prevent camera from going through objects.
  - Ray casting.

![Screenshot](/screenshot.png?raw=true)