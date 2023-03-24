pub trait Shader {
    fn apply<'a, 'b>(&'a mut self, pass: &mut wgpu::RenderPass<'b>) where 'a: 'b;
}
