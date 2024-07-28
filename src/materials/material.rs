use crate::components::{Camera, Transform};
use crate::graphics::Graphics;

pub trait Material {
    fn apply<'a>(
        &'a mut self,
        encoder: &mut wgpu::RenderBundleEncoder<'a>,
        gfx: &Graphics,
        camera: (&Camera, &Transform),
        transform: &Transform,
    );
}
