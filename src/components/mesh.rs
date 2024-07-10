use std::sync::Arc;

// TODO Avoid Arc? It's needed because apparently components must be thread-safe.
pub struct Mesh(pub Arc<crate::assets::Mesh>);
