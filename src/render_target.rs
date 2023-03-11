use winit::dpi::PhysicalSize;
use crate::device::Device;
use crate::texture::Texture;

pub struct RenderTarget {
    color_tex: Texture,
    depth_tex: Texture,
}

impl RenderTarget {
    pub fn new(gfx: &Device, size: Option<PhysicalSize<u32>>) -> Self {
        let size = size.unwrap_or(gfx.surface_size());
        let color_tex = Texture::new_render_attachment(gfx, size);
        let depth_tex = Texture::new_depth(&gfx, size);

        Self {
            color_tex,
            depth_tex,
        }
    }

    pub fn color_tex(&self) -> &Texture { &self.color_tex }
    pub fn depth_tex(&self) -> &Texture { &self.depth_tex }
}