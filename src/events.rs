use bevy_ecs::prelude::Event;

use crate::resources::SurfaceSize;

#[derive(Event)]
pub struct ResizeEvent(pub SurfaceSize);
