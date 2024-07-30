use hecs::World;

use crate::input::{Input, InputAction};
use crate::math::Vec3;
use crate::physics::Physics;

use super::player::Player;
use super::RigidBody;

pub struct Grab {
    // Original distance from the player when the grab was triggered.
    distance: f32,
    // Vector between the body position and the grab point at the moment of grab, both in global coordinates.
    offset: Vec3,
}

impl Grab {
    pub fn update(world: &mut World, input: &Input, physics: &mut Physics) {
        fn release_grab(world: &mut World, physics: &mut Physics) {
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

        let (player_focus, player_focus_ray) = {
            let (_, player) = world.query_mut::<&Player>().into_iter().next().unwrap();
            (player.focus(), player.focus_ray())
        };

        if input.action_active(InputAction::Grab) {
            // Nothing grabbed yet?
            if world.query::<&Grab>().iter().next().is_none() {
                // Init a new grab if there's something in player's focus
                if let Some(player_focus) = player_focus {
                    let body_entity = {
                        let mut q = world.query::<&RigidBody>();
                        let (body_entity, body) = q
                            .into_iter()
                            .find(|(_, body)| body.handle() == player_focus.body)
                            .unwrap();
                        body.set_kinematic(physics, true);
                        body_entity
                    };
                    let body = physics.bodies.get_mut(player_focus.body).unwrap();
                    let offset = *body.translation() - player_focus.point;
                    world
                        .insert(
                            body_entity,
                            (Grab {
                                distance: player_focus.distance,
                                offset,
                            },),
                        )
                        .unwrap();
                }
            } else {
                // Update the grabbed object
                let existing_grab = if let Some((_, (grab, body))) =
                    world.query::<(&Grab, &RigidBody)>().iter().next()
                {
                    Some((grab.distance, grab.offset, body.handle()))
                } else {
                    None
                };

                if let Some(player_focus_ray) = player_focus_ray {
                    if let Some((grab_distance, grab_offset, body)) = existing_grab {
                        let new_pos = player_focus_ray.point_at(grab_distance) + grab_offset;
                        physics
                            .bodies
                            .get_mut(body)
                            .unwrap()
                            .set_translation(new_pos.coords, true);
                    }
                } else {
                    release_grab(world, physics);
                }
            }
        } else {
            release_grab(world, physics);
        }
    }
}
