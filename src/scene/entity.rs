use crate::camera::Camera;
use crate::device::{Device, Frame};
use crate::physics::PhysicsWorld;

pub trait Entity {
    fn update(&mut self, dt: f32, physics: &PhysicsWorld);

    fn render<'a, 'b>(
        &'a mut self,
        gfx: &'a Device,
        camera: &'a Camera, // TODO Avoid
        frame: &mut Frame<'b>
    ) where 'a: 'b;
}