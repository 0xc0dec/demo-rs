pub trait Shader {
    fn apply<'a>(&'a mut self, pass: &mut wgpu::RenderPass<'a>);
}
