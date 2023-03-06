use wgpu::RenderPass;
use crate::camera::Camera;
use crate::driver::Driver;
use crate::physics::PhysicsWorld;

pub trait SceneNode {
    fn update(&mut self, dt: f32, physics: &PhysicsWorld);

    fn render<'a, 'b>(
        &'a mut self,
        driver: &'a Driver,
        camera: &'a Camera, // TODO Avoid
        pass: &mut RenderPass<'b>
    ) where 'a: 'b;
}