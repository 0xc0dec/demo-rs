# About

A simple graphics demo where you can fly around, spawn boxes and grab them.

![Demo](/screenshot.png?raw=true)

I made this project to learn Rust and try it in game development. There is no attempt to create an "engine", everything
is pretty low level and abstractions are built along the way when needed.

## Building and running

```
cargo run
```

Check the on-screen tip for controls.

## Features

- [wgpu](https://github.com/gfx-rs/wgpu) rendering.
- [nalgebra](https://github.com/dimforge/nalgebra) math.
- [Rapier](https://rapier.rs) physics
    - Rigid bodies with colliders.
    - Camera with character controller, preventing it from passing through objects.
    - Ray casting.
    - Drag-n-drop.
- First person flying camera ("spectator") with clamping of vertical angles to protect from overturning.
- Skybox rendering on a full-screen quad.
- Vignette post-processing.
