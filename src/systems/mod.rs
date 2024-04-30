use bevy_ecs::prelude::*;
use winit::event::VirtualKeyCode;

pub use consume_system_events::consume_system_events;
pub use init_app::init_app;
pub use render::render;
pub use schedules::*;
pub use update_and_build_debug_ui::update_and_build_debug_ui;
pub use update_input_state::update_input_state;

use crate::resources::{App, Device, FrameTime, PhysicsWorld};
use crate::resources::events::{KeyboardEvent, WindowResizeEvent};

mod consume_system_events;
mod init_app;
mod render;
mod schedules;
mod update_and_build_debug_ui;
mod update_input_state;

// TODO Combine these systems?

pub fn resize_device(mut device: ResMut<Device>, mut events: EventReader<WindowResizeEvent>) {
    if let Some(e) = events.read().last() {
        device.resize(e.new_size)
    }
}

pub fn escape_on_exit(mut app: ResMut<App>, mut keyboard_events: EventReader<KeyboardEvent>) {
    if keyboard_events
        .read()
        .any(|e| e.code == VirtualKeyCode::Escape && e.pressed)
    {
        app.running = false;
    }
}

pub fn update_physics(mut physics: ResMut<PhysicsWorld>, frame_time: Res<FrameTime>) {
    physics.update(frame_time.delta);
}

pub fn update_frame_time(mut frame_time: ResMut<FrameTime>) {
    frame_time.update();
}
