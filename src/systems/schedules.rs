use bevy_ecs::prelude::{run_once, Condition, IntoSystemConfigs, Query, Schedule};
use bevy_ecs::schedule::ScheduleLabel;

use crate::components::{
    FloorBox, FreeBox, Grabbed, PhysicsBody, Player, PlayerTarget, PostProcessor, Skybox,
};

use super::build_debug_ui::build_debug_ui;
use super::misc::update_physics;

#[derive(ScheduleLabel, Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct SpawnSceneSchedule;

pub fn new_spawn_scene_schedule() -> (Schedule, SpawnSceneSchedule) {
    let mut schedule = Schedule::new(SpawnSceneSchedule {});
    schedule
        .add_systems(
            (
                // Assets::load,
                Skybox::spawn,
                FreeBox::spawn_sample,
                FloorBox::spawn,
                Player::spawn,
                PlayerTarget::spawn,
            )
                .run_if(run_once()),
        )
        .add_systems(
            PostProcessor::spawn
                .run_if((|player: Query<&Player>| !player.is_empty()).and_then(run_once())),
        );
    (schedule, SpawnSceneSchedule)
}

#[derive(ScheduleLabel, Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct UpdateSchedule;

pub fn new_update_schedule() -> (Schedule, UpdateSchedule) {
    let mut schedule = Schedule::new(UpdateSchedule {});
    schedule
        .add_systems(update_physics)
        .add_systems(PhysicsBody::sync.after(update_physics))
        .add_systems(Player::update.after(update_physics))
        .add_systems(PlayerTarget::update.after(Player::update))
        .add_systems(Grabbed::update.after(Player::update))
        .add_systems(FreeBox::spawn_by_player.after(Player::update))
        .add_systems(PostProcessor::update.after(Player::update))
        .add_systems(build_debug_ui);
    (schedule, UpdateSchedule)
}
