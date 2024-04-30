use bevy_ecs::prelude::{Commands, Component, Entity, Query, Res, ResMut};

use crate::components::*;
use crate::math::{to_point, Vec3};
use crate::resources::{Events, PhysicsWorld};

#[derive(Component)]
pub struct Grab {
    // Coordinates of the body in Player's local coord. system
    pub body_local_pos: Vec3,
}

impl Grab {
    pub fn grab_or_release(
        player: Query<(&Player, &Transform)>,
        bodies: Query<(Entity, &PhysicsBody)>,
        grab: Query<(Entity, &Grab)>,
        events: Res<Events>,
        physics: ResMut<PhysicsWorld>,
        mut commands: Commands,
    ) {
        let (player, player_transform) = player.single();

        if events.lmb_down {
            if grab.is_empty() {
                if let Some(target_body) = player.target_body() {
                    let (body_entity, ..) = bodies
                        .iter()
                        .find(|(_, b)| b.body_handle() == target_body)
                        .unwrap();

                    let body = physics.bodies.get(target_body).unwrap();
                    let body_local_pos = player_transform
                        .matrix()
                        .try_inverse()
                        .unwrap()
                        .transform_point(&to_point(*body.translation()))
                        .coords;

                    commands
                        .entity(body_entity)
                        .insert((Grab { body_local_pos },));
                }
            }
        } else {
            for g in grab.iter() {
                commands.entity(g.0).remove::<Grab>();
            }
        }
    }
}
