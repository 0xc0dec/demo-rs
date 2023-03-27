use crate::assets::Assets;
use crate::debug_ui::DebugUI;
use crate::device::{Device, SurfaceSize};
use crate::events::{KeyboardEvent, MouseEvent, WindowResizeEvent};
use crate::frame_time::FrameTime;
use crate::input_state::InputState;
use crate::physics_world::PhysicsWorld;
use crate::app_state::AppState;
use bevy_ecs::prelude::*;
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;

pub fn init_app(world: &mut World) {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Demo")
        .with_inner_size(SurfaceSize::new(1800, 1200))
        .build(&event_loop)
        .unwrap();

    let device = pollster::block_on(async {
        Device::new(&window).await
    });

    let assets = Assets::new();
    let input = InputState::new();
    let physics = PhysicsWorld::new();
    let debug_ui = DebugUI::new(&device, &window);

    world.insert_resource(AppState {
        running: true,
        frame_time: FrameTime::new(),
    });
    world.insert_resource(input);

    world.init_resource::<Events<WindowResizeEvent>>();
    world.init_resource::<Events<KeyboardEvent>>();
    world.init_resource::<Events<MouseEvent>>();

    world.insert_non_send_resource(physics);
    world.insert_non_send_resource(window);
    world.insert_non_send_resource(event_loop);
    world.insert_non_send_resource(device);
    world.insert_non_send_resource(assets);
    world.insert_non_send_resource(debug_ui);
}