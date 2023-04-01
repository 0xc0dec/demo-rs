use bevy_ecs::prelude::{Component, Query, Res, ResMut};
use rapier3d::prelude::RigidBodyHandle;
use crate::components::{PhysicsBody, Player, Transform};
use crate::input::Input;
use crate::math::{Quat, to_point, Vec3};
use crate::physics_world::PhysicsWorld;

#[derive(Component)]
pub struct Grab {
    target_handle: Option<RigidBodyHandle>,
    // Coordinates of the body in Player's local coord. system
    body_local_pos: Vec3,
    // Rotation of the body in Player's local coord. system
    body_local_rot: Quat
}

impl Grab {
    pub fn new() -> Grab {
        Self {
            target_handle: None,
            body_local_pos: Vec3::zeros(),
            body_local_rot: Quat::identity()
        }
    }

    pub fn update(
        mut grab: Query<&mut Grab>,
        player: Query<(&Player, &Transform)>,
        bodies: Query<&PhysicsBody>,
        input: Res<Input>,
        mut physics: ResMut<PhysicsWorld>
    ) {
        let mut grab = grab.single_mut();
        let (player, player_transform) = player.single();

        if input.lmb_down {
            if let Some(target_handle) = grab.target_handle {
                let new_pos = player_transform.matrix()
                    .transform_point(&to_point(grab.body_local_pos));
                bodies.iter()
                    .find(|b| b.body_handle() == target_handle).unwrap()
                    .move_to(new_pos.coords, &mut physics);
            } else if let Some(target_body) = player.target_body() {
                grab.target_handle = Some(target_body);

                let body = physics.bodies.get(target_body).unwrap();
                let (body_pos, _body_rot) = (body.translation(), body.rotation());
                let player_inv_matrix = player_transform.matrix()
                    .try_inverse()
                    .unwrap();

                grab.body_local_pos = player_inv_matrix
                    .transform_point(&to_point(*body_pos))
                    .coords;

                bodies.iter()
                    .find(|b| b.body_handle() == target_body).unwrap()
                    .grab(&mut physics);
            }
        } else if let Some(target_handle) = grab.target_handle {
            // TODO Avoid this copypasta
            bodies.iter()
                .find(|b| b.body_handle() == target_handle).unwrap()
                .release(&mut physics);
            grab.target_handle = None
        }
    }
}