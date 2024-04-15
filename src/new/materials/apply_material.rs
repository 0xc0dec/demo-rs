use crate::new::camera::Camera;
use crate::new::Device;
use crate::new::transform::Transform;

// TODO Better name
pub trait ApplyMaterial {
    fn apply<'a>(
        &'a mut self,
        encoder: &mut wgpu::RenderBundleEncoder<'a>,
        device: &Device,
        camera: (&Camera, &Transform),
        transform: &Transform,
    );
}
