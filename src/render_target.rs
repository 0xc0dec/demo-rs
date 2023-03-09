use crate::graphics::Graphics;
use crate::texture::Texture;

pub struct RenderTarget {
    color_tex: Texture,
    depth_tex: Texture,
}

impl RenderTarget {
    pub fn new(gfx: &Graphics) -> Self {
        let color_tex = Texture::new_render_attachment(gfx, gfx.surface_size());
        let depth_tex = Texture::new_depth(&gfx, gfx.surface_size());

        Self {
            color_tex,
            depth_tex,
        }
    }

    pub fn color_tex(&self) -> &Texture { &self.color_tex }
    pub fn depth_tex(&self) -> &Texture { &self.depth_tex }
}