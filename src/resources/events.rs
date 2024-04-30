use bevy_ecs::prelude::Event;

use crate::resources::SurfaceSize;

#[derive(Event)]
pub struct WindowResizeEvent {
    pub new_size: SurfaceSize,
}
