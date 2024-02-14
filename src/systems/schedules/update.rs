use bevy_ecs::prelude::Schedule;
use bevy_ecs::schedule::ScheduleLabel;

use crate::components::{FreeBox, Grab, PhysicsBody, Player, PlayerTarget, PostProcessor};
use crate::systems::*;

#[derive(ScheduleLabel, Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Label;

pub fn new_update_schedule() -> (Schedule, Label) {
    let mut schedule = Schedule::new(Label {});
    schedule
        .add_systems(update_physics)
        .add_systems(PhysicsBody::sync.after(update_physics))
        .add_systems(Player::update.after(update_physics))
        .add_systems(PlayerTarget::update.after(Player::update))
        .add_systems(Grab::grab_or_release.after(Player::update))
        .add_systems(PhysicsBody::grab_start_stop.after(Player::update))
        .add_systems(PhysicsBody::update_grabbed.after(PhysicsBody::grab_start_stop))
        .add_systems(FreeBox::spawn_by_player.after(Player::update))
        .add_systems(PostProcessor::update.after(Player::update))
        .add_systems(update_and_build_debug_ui.after(update_physics));
    (schedule, Label)
}
