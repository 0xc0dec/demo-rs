use crate::device::Device;
use crate::texture::{Texture, TextureSize};

pub struct RenderTarget {
    color_tex: Texture,
    depth_tex: Texture,
}

impl RenderTarget {
    pub fn new(device: &Device, size: Option<TextureSize>) -> Self {
        let size = size.unwrap_or(device.surface_size().into());
        let color_tex = Texture::new_render_attachment(
            device.device(), device.surface_texture_format(), size.into()
        );
        let depth_tex = Texture::new_depth(
            device.device(), device.depth_texture_format(), size.into()
        );

        Self {
            color_tex,
            depth_tex,
        }
    }

    pub fn color_tex(&self) -> &Texture {
        &self.color_tex
    }
    pub fn depth_tex(&self) -> &Texture {
        &self.depth_tex
    }
}
