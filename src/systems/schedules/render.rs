use bevy_ecs::prelude::Schedule;
use bevy_ecs::schedule::ScheduleLabel;

use crate::systems::*;

#[derive(ScheduleLabel, Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Label;

pub fn new_render_schedule() -> (Schedule, Label) {
    let mut schedule = Schedule::new(Label {});
    schedule.add_systems(render);
    (schedule, Label)
}
