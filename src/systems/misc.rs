use bevy_ecs::change_detection::{Res, ResMut};

use crate::resources::{FrameTime, PhysicsWorld};

// TODO Remove
pub fn update_physics(mut physics: ResMut<PhysicsWorld>, frame_time: Res<FrameTime>) {
    physics.update(frame_time.delta);
}
