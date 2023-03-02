use crate::renderer::Renderer;
use crate::texture::Texture;

pub struct RenderTarget {
    depth_texture: Texture,
}

impl RenderTarget {
    pub fn new(renderer: &Renderer) -> Self {
        let depth_texture = Texture::new_depth_texture(renderer);

        RenderTarget {
            depth_texture
        }
    }

    pub fn resize(&mut self, renderer: &Renderer) {
        self.depth_texture = Texture::new_depth_texture(renderer);
    }

    pub fn depth_texture(&self) -> &Texture { &self.depth_texture }
}