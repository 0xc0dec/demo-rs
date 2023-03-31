use bevy_ecs::prelude::{Component, Query, Res, ResMut};
use rapier3d::na::Point3;
use rapier3d::prelude::RigidBodyHandle;
use crate::components::{PhysicsBody, Player, Transform};
use crate::input::Input;
use crate::math::Vec3;
use crate::physics_world::PhysicsWorld;

#[derive(Component)]
pub struct Grab {
    body_handle: Option<RigidBodyHandle>,
    offset: Vec3
}

impl Grab {
    pub fn new() -> Grab {
        Self {
            body_handle: None,
            offset: Vec3::zeros()
        }
    }

    pub fn update(
        mut grab: Query<&mut Grab>,
        player: Query<(&Player, &Transform)>,
        mut bodies: Query<&mut PhysicsBody>,
        input: Res<Input>,
        mut physics: ResMut<PhysicsWorld>
    ) {
        let mut grab = grab.single_mut();
        let (player, player_transform) = player.single();

        if input.lmb_down {
            if let Some(body_handle) = grab.body_handle {
                for body in bodies.iter_mut() {
                    if body.body_handle() == body_handle {
                        body.move_to(player_transform.position() + grab.offset, &mut physics)
                    }
                }
            } else if let Some(target_body) = player.target_body() {
                grab.body_handle = Some(target_body);
                grab.offset = player_transform
                    .matrix().try_inverse().unwrap()
                    .transform_point(&Point3::from(player.target_pt().unwrap()))
                    .coords;

                for body in bodies.iter_mut() {
                    if body.body_handle() == target_body {
                        body.grab(&mut physics)
                    }
                }
            }
        } else if let Some(body_handle) = grab.body_handle {
            for body in bodies.iter_mut() {
                if body.body_handle() == body_handle {
                    body.release(&mut physics);
                }
            }
            grab.body_handle = None
        }
    }
}