use bevy_ecs::prelude::{Commands, Component, Entity, Query, Res, ResMut, Without};

use crate::components::*;
use crate::math::{to_point, Vec3};
use crate::resources::{Input, InputAction, PhysicsWorld};

#[derive(Component)]
pub struct Grabbed {
    // Coordinates of the body in Player's local coord. system
    pub body_local_pos: Vec3,
}

impl Grabbed {
    pub fn update(
        input: Res<Input>,
        player: Query<(&Player, &Transform)>,
        mut free_bodies: Query<(Entity, &mut PhysicsBody), Without<Grabbed>>,
        mut grabbed: Query<(Entity, &mut PhysicsBody, &Grabbed)>,
        mut physics: ResMut<PhysicsWorld>,
        mut commands: Commands,
    ) {
        let (player, player_transform) = player.single();

        if input.action_active(InputAction::Grab) {
            if grabbed.is_empty() {
                // Initiate grab
                if let Some(target_body_handle) = player.target_body() {
                    let (body_entity, body) = free_bodies
                        .iter_mut()
                        .find(|(_, b)| b.body_handle() == target_body_handle)
                        .unwrap();

                    body.set_kinematic(&mut physics, true);
                    let body = physics.bodies.get_mut(target_body_handle).unwrap();

                    let body_local_pos = player_transform
                        .matrix()
                        .try_inverse()
                        .unwrap()
                        .transform_point(&to_point(*body.translation()))
                        .coords;

                    commands
                        .entity(body_entity)
                        .insert((Grabbed { body_local_pos },));
                }
            } else {
                // Update the grabbed thing
                if let Ok((_, body, grabbed)) = grabbed.get_single() {
                    let body = physics.bodies.get_mut(body.body_handle()).unwrap();
                    let new_pos = player_transform
                        .matrix()
                        .transform_point(&to_point(grabbed.body_local_pos));
                    body.set_translation(new_pos.coords, true);
                }
            }
        } else {
            // Release grab
            for (entity, body, _) in grabbed.iter_mut() {
                body.set_kinematic(&mut physics, false);
                commands.entity(entity).remove::<Grabbed>();
            }
        }
    }
}
