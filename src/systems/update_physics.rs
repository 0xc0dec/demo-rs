use bevy_ecs::prelude::{NonSendMut, Res};
use crate::physics_world::PhysicsWorld;
use crate::state::State;

pub fn update_physics(mut physics: NonSendMut<PhysicsWorld>, state: Res<State>) {
    physics.update(state.frame_time.delta);
}