use bevy_ecs::prelude::*;
use std::sync::Arc;

// TODO Avoid Arc? It's needed because apparently components must be thread-safe.
#[derive(Component)]
pub struct Mesh(pub Arc<crate::assets::Mesh>);
