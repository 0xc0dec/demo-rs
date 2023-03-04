use crate::renderer::Renderer;
use crate::texture::Texture;

pub struct RenderTarget {
    // TODO Add color texture
    depth_tex: Texture,
}

impl RenderTarget {
    pub fn new(renderer: &Renderer) -> Self {
        let depth_tex = Texture::new_depth_texture(renderer);

        RenderTarget {
            depth_tex
        }
    }

    pub fn resize(&mut self, renderer: &Renderer) {
        self.depth_tex = Texture::new_depth_texture(renderer);
    }

    pub fn depth_texture(&self) -> &Texture { &self.depth_tex }
}