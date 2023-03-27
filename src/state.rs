use crate::frame_time::FrameTime;
use bevy_ecs::prelude::*;

// TODO Rename to smth more specific like app_state
#[derive(Resource)]
pub struct State {
    pub running: bool,
    pub frame_time: FrameTime,
}
