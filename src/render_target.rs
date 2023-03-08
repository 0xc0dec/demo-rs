use crate::graphics::Graphics;
use crate::texture::Texture;

pub struct RenderTarget {
    // TODO Add color texture
    depth_tex: Texture,
    clear_color: wgpu::Color,
}

impl RenderTarget {
    pub fn new(gfx: &Graphics, clear_color: wgpu::Color) -> Self {
        let depth_tex = Texture::depth(gfx);

        RenderTarget {
            depth_tex,
            clear_color
        }
    }

    pub fn resize(&mut self, gfx: &Graphics) {
        self.depth_tex = Texture::depth(gfx);
    }

    pub fn depth_texture(&self) -> &Texture { &self.depth_tex }
    pub fn clear_color(&self) -> wgpu::Color { self.clear_color }
}