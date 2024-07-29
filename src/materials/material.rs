use crate::components::{Camera, Transform};
use crate::graphics::Graphics;

pub trait Material {
    fn update(
        &mut self,
        gfx: &Graphics,
        _camera: &Camera,
        _camera_transform: &Transform,
        _transform: &Transform,
    ) {
    }

    fn apply<'a>(&'a self, encoder: &mut wgpu::RenderBundleEncoder<'a>);
}
