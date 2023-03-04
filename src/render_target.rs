use crate::driver::Driver;
use crate::texture::Texture;

pub struct RenderTarget {
    // TODO Add color texture
    depth_tex: Texture,
}

impl RenderTarget {
    pub fn new(driver: &Driver) -> Self {
        let depth_tex = Texture::depth(driver);

        RenderTarget {
            depth_tex
        }
    }

    pub fn resize(&mut self, driver: &Driver) {
        self.depth_tex = Texture::depth(driver);
    }

    pub fn depth_texture(&self) -> &Texture { &self.depth_tex }
}