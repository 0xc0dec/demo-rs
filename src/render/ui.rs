use super::renderer::Renderer;

pub trait Ui {
    fn draw<'a>(&'a mut self, rr: &Renderer, encoder: &mut wgpu::RenderPass<'a>);
}
