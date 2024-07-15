use crate::camera::Camera;
use crate::graphics::Graphics;
use crate::transform::Transform;

// TODO Better name
pub trait ApplyMaterial {
    fn apply<'a>(
        &'a mut self,
        encoder: &mut wgpu::RenderBundleEncoder<'a>,
        gfx: &Graphics,
        camera: (&Camera, &Transform),
        transform: &Transform,
    );
}
