use crate::device::{Device, SurfaceSize};
use crate::texture::Texture;

pub struct RenderTarget {
    color_tex: Texture,
    depth_tex: Texture,
}

impl RenderTarget {
    pub fn new(device: &Device, size: Option<SurfaceSize>) -> Self {
        let size = size.unwrap_or(device.surface_size());
        let color_tex = Texture::new_render_attachment(device, size.into());
        let depth_tex = Texture::new_depth(&device, size.into());

        Self {
            color_tex,
            depth_tex,
        }
    }

    pub fn color_tex(&self) -> &Texture { &self.color_tex }
    pub fn depth_tex(&self) -> &Texture { &self.depth_tex }
}