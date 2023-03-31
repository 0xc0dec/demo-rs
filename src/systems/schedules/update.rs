use bevy_ecs::prelude::{Schedule};
use bevy_ecs::schedule::ScheduleLabel;
use crate::components::{PhysicsBody, Player, PostProcessor, PlayerTarget, Grab};
use crate::systems::*;

#[derive(ScheduleLabel, Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Label;

pub fn new_update_schedule() -> (Schedule, Label) {
    let mut schedule = Schedule::default();
    schedule
        .add_system(update_physics)
        .add_system(PhysicsBody::sync.after(update_physics))
        .add_system(Player::update.after(update_physics))
        .add_system(PlayerTarget::update.after(Player::update))
        .add_system(Grab::update.after(Player::update))
        .add_system(PostProcessor::update.after(Player::update))
        .add_system(update_and_build_debug_ui.after(update_physics));
    (schedule, Label)
}