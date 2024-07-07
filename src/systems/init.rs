use bevy_ecs::prelude::*;
use winit::dpi::PhysicalSize;
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;

use crate::debug_ui::DebugUI;
use crate::events::{KeyboardEvent, MouseEvent, ResizeEvent};
use crate::resources::{App, Device, FrameTime, Input, PhysicsWorld};

pub fn init(world: &mut World) {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Demo")
        .with_inner_size(PhysicalSize {
            width: 1900,
            height: 1200,
        })
        .build(&event_loop)
        .unwrap();
    let device = pollster::block_on(Device::new(&window));

    world.init_resource::<Events<ResizeEvent>>();
    world.init_resource::<Events<MouseEvent>>();
    world.init_resource::<Events<KeyboardEvent>>();

    world.insert_non_send_resource(event_loop);
    world.insert_non_send_resource(DebugUI::new(&device, &window));
    world.insert_non_send_resource(window);

    world.insert_resource(App::new());
    world.insert_resource(device);
    world.insert_resource(FrameTime::new());
    world.insert_resource(Input::new());
    world.insert_resource(PhysicsWorld::new());
}
