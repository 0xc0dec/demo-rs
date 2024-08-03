# About

A simple graphics demo where you can fly around, spawn boxes and grab them.

![Demo](/screenshot.png?raw=true)

I made this project to learn Rust and try it in game development. There is no attempt to create an "engine", everything
is pretty low level and abstractions are built along the way when needed.

Even though this is a learning project, the code is not heavily commented.
Hopefully it's clean enough to be readable and easy to navigate.

## Building and running

```
cargo run
```

Controls:

- Toggle camera control: `Tab`
- Move: `WASDQE`
- Grab/drop boxes: left mouse click
- Spawn new box: `F`
- Quit: `Esc`

## Features

- [hecs](https://github.com/Ralith/hecs) ECS.
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
