use bevy_ecs::prelude::World;
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;
use crate::assets::Assets;
use crate::debug_ui::DebugUI;
use crate::device::{Device, SurfaceSize};
use crate::frame_time::FrameTime;
use crate::input::Input;
use crate::physics_world::PhysicsWorld;
use crate::state::State;

pub fn init(world: &mut World) {
    pollster::block_on(async {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_title("Demo")
            .with_inner_size(SurfaceSize::new(1800, 1200))
            .build(&event_loop)
            .unwrap();
        let device = Device::new(&window).await;
        let assets = Assets::new();
        let input = Input::new();
        let physics = PhysicsWorld::new();
        let debug_ui = DebugUI::new(&device, &window);

        world.insert_resource(State { running: true, frame_time: FrameTime::new() });
        world.insert_non_send_resource(physics);
        world.insert_non_send_resource(window);
        world.insert_non_send_resource(event_loop);
        world.insert_non_send_resource(device);
        world.insert_non_send_resource(assets);
        world.insert_non_send_resource(input);
        world.insert_non_send_resource(debug_ui);
    });
}
