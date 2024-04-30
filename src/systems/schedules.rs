use bevy_ecs::prelude::{Condition, IntoSystemConfigs, Query, run_once, Schedule};
use bevy_ecs::schedule::ScheduleLabel;

use crate::components::{
    FloorBox, FreeBox, Grab, PhysicsBody, Player, PlayerTarget, PostProcessor, Skybox,
};
use crate::resources::Assets;

use super::capture_events::capture_events;
use super::escape_on_exit;
use super::render::render;
use super::resize_device;
use super::update_and_build_debug_ui::update_and_build_debug_ui;
use super::update_frame_time;
use super::update_physics;

#[derive(ScheduleLabel, Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct BeforeUpdateSchedule;

pub fn new_before_update_schedule() -> (Schedule, BeforeUpdateSchedule) {
    let mut schedule = Schedule::new(BeforeUpdateSchedule {});
    schedule
        .add_systems(capture_events)
        .add_systems((escape_on_exit, resize_device, update_frame_time).after(capture_events));
    (schedule, BeforeUpdateSchedule)
}

#[derive(ScheduleLabel, Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct RenderSchedule;

pub fn new_render_schedule() -> (Schedule, RenderSchedule) {
    let mut schedule = Schedule::new(RenderSchedule {});
    schedule.add_systems(render);
    (schedule, RenderSchedule)
}

#[derive(ScheduleLabel, Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct SpawnSceneSchedule;

pub fn new_spawn_scene_schedule() -> (Schedule, SpawnSceneSchedule) {
    let mut schedule = Schedule::new(SpawnSceneSchedule {});
    schedule
        .add_systems(
            (
                Assets::load,
                Skybox::spawn,
                FreeBox::spawn,
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
        .add_systems(Grab::grab_or_release.after(Player::update))
        .add_systems(PhysicsBody::grab_start_stop.after(Player::update))
        .add_systems(PhysicsBody::update_grabbed.after(PhysicsBody::grab_start_stop))
        .add_systems(FreeBox::spawn_by_player.after(Player::update))
        .add_systems(PostProcessor::update.after(Player::update))
        .add_systems(update_and_build_debug_ui.after(update_physics));
    (schedule, UpdateSchedule)
}
