use crate::device::Device;

use super::super::{Camera, Transform};

// TODO Better name
pub trait ApplyMaterial: Sync + Send {
    fn apply<'a>(
        &'a mut self,
        encoder: &mut wgpu::RenderBundleEncoder<'a>,
        device: &Device,
        camera: (&Camera, &Transform),
        transform: &Transform,
    );
}
