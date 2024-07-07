use bevy_ecs::prelude::*;

#[derive(Resource)]
pub struct App {
    pub running: bool,
}

impl App {
    pub fn new() -> Self {
        Self { running: true }
    }
}
