use bevy_ecs::change_detection::{Res, ResMut};
use bevy_ecs::event::EventReader;
use winit::event::VirtualKeyCode;

use crate::events::ResizeEvent;
use crate::resources::{App, Device, FrameTime, Input, PhysicsWorld};

pub fn resize_device(mut device: ResMut<Device>, mut events: EventReader<ResizeEvent>) {
    if let Some(e) = events.read().last() {
        device.resize(e.0);
    }
}

pub fn escape_on_exit(mut app: ResMut<App>, input: Res<Input>) {
    if input.key_pressed(VirtualKeyCode::Escape) {
        app.running = false;
    }
}

pub fn update_physics(mut physics: ResMut<PhysicsWorld>, frame_time: Res<FrameTime>) {
    physics.update(frame_time.delta);
}

pub fn update_frame_time(mut frame_time: ResMut<FrameTime>) {
    frame_time.update();
}
