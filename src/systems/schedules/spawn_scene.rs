use bevy_ecs::prelude::*;
use bevy_ecs::schedule::ScheduleLabel;

use crate::assets::Assets;
use crate::components::{FloorBox, FreeBox, Player, PlayerTarget, PostProcessor, Skybox};

#[derive(ScheduleLabel, Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Label;

pub fn new_spawn_scene_schedule() -> (Schedule, Label) {
    let mut schedule = Schedule::new(Label {});
    schedule
        .add_systems(Assets::load.run_if(run_once()))
        .add_systems(Skybox::spawn.run_if(run_once()))
        .add_systems(FreeBox::spawn.run_if(run_once()))
        .add_systems(FloorBox::spawn.run_if(run_once()))
        .add_systems(Player::spawn.run_if(run_once()))
        .add_systems(PlayerTarget::spawn.run_if(run_once()))
        .add_systems(
            PostProcessor::spawn
                .run_if((|player: Query<&Player>| !player.is_empty()).and_then(run_once())),
        );
    (schedule, Label)
}
