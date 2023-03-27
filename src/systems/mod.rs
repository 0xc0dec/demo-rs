mod update_and_build_debug_ui;
mod handle_system_events;
mod init_app;
mod render;
mod update_input_state;
mod grab_cursor;
mod schedules;

use crate::device::Device;
use crate::events::{KeyboardEvent, WindowResizeEvent};
use crate::physics_world::PhysicsWorld;
use crate::app::App;
use bevy_ecs::prelude::*;
use winit::event::{VirtualKeyCode};

pub use update_and_build_debug_ui::update_and_build_debug_ui;
pub use handle_system_events::handle_system_events;
pub use init_app::init_app;
pub use render::render;
pub use update_input_state::update_input_state;
pub use grab_cursor::grab_cursor;
pub use schedules::*;
use crate::frame_time::FrameTime;

pub fn resize_device(mut device: NonSendMut<Device>, mut events: EventReader<WindowResizeEvent>) {
    events.iter().last().map(|e| device.resize(e.new_size));
}

pub fn escape_on_exit(mut app: ResMut<App>, mut keyboard_events: EventReader<KeyboardEvent>) {
    if keyboard_events
        .iter()
        .any(|e| e.code == VirtualKeyCode::Escape && e.pressed)
    {
        app.running = false;
    }
}

pub fn update_physics(mut physics: NonSendMut<PhysicsWorld>, frame_time: Res<FrameTime>) {
    physics.update(frame_time.delta);
}

pub fn update_frame_time(mut frame_time: ResMut<FrameTime>) {
    frame_time.update();
}
