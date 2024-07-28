use hecs::World;

use crate::input::{Input, InputAction};
use crate::math::{to_point3, Vec3};
use crate::physics::Physics;

use super::player::Player;
use super::RigidBody;
use super::transform::Transform;

pub struct Grab {
    pos_relative_to_player: Vec3,
}

impl Grab {
    pub fn update(world: &mut World, input: &Input, physics: &mut Physics) {
        let (player_focus_at_body, player_tr_matrix) = {
            let mut q = world.query::<(&Player, &Transform)>();
            let (_, (player, player_tr)) = q.iter().next().unwrap();
            (player.focus_body(), player_tr.matrix())
        };

        if input.action_active(InputAction::Grab) {
            // Nothing grabbed yet
            if world.query::<&Grab>().iter().next().is_none() {
                // Init a new grab if player is looking at something
                if let Some(focus_body) = player_focus_at_body {
                    let body_entity = {
                        let mut q = world.query::<&RigidBody>();
                        let (body_entity, body) = q
                            .into_iter()
                            .find(|(_, body)| body.body_handle() == focus_body)
                            .unwrap();
                        body.set_kinematic(physics, true);
                        body_entity
                    };
                    let body = physics.bodies.get_mut(focus_body).unwrap();
                    let local_pos = player_tr_matrix
                        .try_inverse()
                        .unwrap()
                        .transform_point(&to_point3(*body.translation()))
                        .coords;
                    world
                        .insert(
                            body_entity,
                            (Grab {
                                pos_relative_to_player: local_pos,
                            },),
                        )
                        .unwrap();
                }
            } else {
                // Update the grabbed object
                if let Some((_, (grab, body))) = world.query::<(&Grab, &RigidBody)>().iter().next()
                {
                    let new_pos =
                        player_tr_matrix.transform_point(&to_point3(grab.pos_relative_to_player));
                    physics
                        .bodies
                        .get_mut(body.body_handle())
                        .unwrap()
                        .set_translation(new_pos.coords, true);
                }
            }
        } else {
            // Release grab
            let entity = if let Some((entity, (_grab, body))) =
                world.query::<(&Grab, &RigidBody)>().iter().next()
            {
                body.set_kinematic(physics, false);
                Some(entity)
            } else {
                None
            };

            if let Some(entity) = entity {
                world.remove_one::<Grab>(entity).unwrap();
            }
        }
    }
}
