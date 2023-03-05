pub trait Material {
    fn apply<'a, 'b>(&'a mut self, pass: &mut wgpu::RenderPass<'b>) where 'a: 'b;
}
