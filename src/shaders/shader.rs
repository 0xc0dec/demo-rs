pub trait Shader {
    fn apply<'a>(&'a mut self, encoder: &mut wgpu::RenderBundleEncoder<'a>);
}
