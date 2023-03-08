use wgpu::RenderPass;
use crate::camera::Camera;
use crate::graphics::Graphics;
use crate::physics::PhysicsWorld;

// TODO Rename to Entity
pub trait SceneNode {
    fn update(&mut self, dt: f32, physics: &PhysicsWorld);

    fn render<'a, 'b>(
        &'a mut self,
        gfx: &'a Graphics,
        camera: &'a Camera, // TODO Avoid
        pass: &mut RenderPass<'b>
    ) where 'a: 'b;
}