use bevy_ecs::prelude::Schedule;
use bevy_ecs::schedule::ScheduleLabel;

use crate::components::{Grabbed, PlayerTarget};

#[derive(ScheduleLabel, Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct UpdateSchedule;

pub fn new_update_schedule() -> (Schedule, UpdateSchedule) {
    let mut schedule = Schedule::new(UpdateSchedule {});
    schedule
        .add_systems(PlayerTarget::update)
        .add_systems(Grabbed::update);
    (schedule, UpdateSchedule)
}
