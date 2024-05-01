use bevy_ecs::change_detection::{Res, ResMut};

use crate::resources::{App, Device, Events, FrameTime, PhysicsWorld};

pub fn resize_device(mut device: ResMut<Device>, events: Res<Events>) {
    if let Some(size) = events.new_surface_size {
        device.resize(size);
    }
}

pub fn escape_on_exit(mut app: ResMut<App>, events: Res<Events>) {
    if events.esc_down {
        app.running = false;
    }
}

pub fn update_physics(mut physics: ResMut<PhysicsWorld>, frame_time: Res<FrameTime>) {
    physics.update(frame_time.delta);
}

pub fn update_frame_time(mut frame_time: ResMut<FrameTime>) {
    frame_time.update();
}
