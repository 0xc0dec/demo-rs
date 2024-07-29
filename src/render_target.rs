use crate::graphics::Graphics;
use crate::texture::{Texture, TextureSize};

pub struct RenderTarget {
    color_tex: Texture,
    depth_tex: Texture,
}

impl RenderTarget {
    pub fn new(gfx: &Graphics, size: Option<TextureSize>) -> Self {
        let size = size.unwrap_or(gfx.surface_size().into());
        let color_tex = Texture::new_render_attachment(gfx, gfx.surface_texture_format(), size);
        let depth_tex = Texture::new_depth(gfx, gfx.depth_texture_format(), size);

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

    pub fn resize(&mut self, new_size: TextureSize, gfx: &Graphics) {
        *self = RenderTarget::new(gfx, Some(new_size));
    }
}
