use bevy_ecs::prelude::*;

pub use consume_system_events::consume_system_events;
pub use init_app::init_app;
pub use render::render;
pub use schedules::*;
pub use update_and_build_debug_ui::update_and_build_debug_ui;

use crate::resources::{App, Device, FrameTime, Input, PhysicsWorld};
use crate::resources::events::WindowResizeEvent;

mod consume_system_events;
mod init_app;
mod render;
mod schedules;
mod update_and_build_debug_ui;

// TODO Combine these systems?

pub fn resize_device(mut device: ResMut<Device>, mut events: EventReader<WindowResizeEvent>) {
    if let Some(e) = events.read().last() {
        device.resize(e.new_size)
    }
}

pub fn escape_on_exit(mut app: ResMut<App>, input: Res<Input>) {
    if input.esc_down {
        app.running = false;
    }
}

pub fn update_physics(mut physics: ResMut<PhysicsWorld>, frame_time: Res<FrameTime>) {
    physics.update(frame_time.delta);
}

pub fn update_frame_time(mut frame_time: ResMut<FrameTime>) {
    frame_time.update();
}
