use bevy_ecs::prelude::Resource;
use crate::frame_time::FrameTime;

#[derive(Resource)]
pub struct State {
    pub running: bool,
    pub frame_time: FrameTime,
}