pub trait Shader<'a, 'b>
where
    'a: 'b,
{
    fn apply(&'a mut self, pass: &mut wgpu::RenderPass<'b>);
}
