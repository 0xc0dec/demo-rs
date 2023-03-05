use wgpu::RenderPass;
use crate::camera::Camera;
use crate::driver::Driver;

pub trait SceneNode {
    fn update(&mut self, dt: f32);
    fn render<'a, 'b>(
        &'a mut self,
        driver: &'a Driver,
        camera: &'a Camera, // TODO Avoid
        pass: &mut RenderPass<'b>
    ) where 'a: 'b;
}