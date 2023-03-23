mod player;

use bevy_ecs::prelude::{Commands, NonSend, NonSendMut};
use bevy_ecs::schedule::Schedule;
pub use player::*;
use crate::camera::Camera;
use crate::device::Device;
use crate::math::Vec3;
use crate::physics_world::PhysicsWorld;

pub struct Scene2;

impl Scene2 {
    pub fn init(
        device: NonSend<Device>,
        mut physics: NonSendMut<PhysicsWorld>,
        mut commands: Commands,
    ) {
        {
            let canvas_size: (f32, f32) = device.surface_size().into();

            commands.spawn((
                Player::new(
                    Camera::new(
                        Vec3::new(10.0, 10.0, 10.0),
                        Vec3::new(0.0, 0.0, 0.0),
                        canvas_size,
                    ),
                    &mut physics,
                ),
            ));
        }
    }

    pub fn configure_update_systems(schedule: &mut Schedule) {
        schedule.add_system(Player::update);
    }
}
