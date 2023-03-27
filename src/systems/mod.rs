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
use crate::state::State;
use bevy_ecs::prelude::*;
use winit::event::{VirtualKeyCode};

pub use update_and_build_debug_ui::update_and_build_debug_ui;
pub use handle_system_events::handle_system_events;
pub use init_app::init_app;
pub use render::render;
pub use update_input_state::update_input_state;
pub use grab_cursor::grab_cursor;
pub use schedules::new_spawn_scene_schedule;

pub fn resize_device(mut device: NonSendMut<Device>, mut events: EventReader<WindowResizeEvent>) {
    for evt in events.iter() {
        device.resize(evt.new_size);
    }
}

pub fn escape_on_exit(mut state: ResMut<State>, mut keyboard_events: EventReader<KeyboardEvent>) {
    if keyboard_events
        .iter()
        .any(|e| e.code == VirtualKeyCode::Escape && e.pressed)
    {
        state.running = false;
    }
}

pub fn update_physics(mut physics: NonSendMut<PhysicsWorld>, state: Res<State>) {
    physics.update(state.frame_time.delta);
}

pub fn update_frame_time(mut state: ResMut<State>) {
    state.frame_time.update();
}
