use bevy_ecs::prelude::{IntoSystemConfigs, Schedule};
use bevy_ecs::schedule::ScheduleLabel;

use crate::systems::*;

#[derive(ScheduleLabel, Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Label;

pub fn new_before_update_schedule() -> (Schedule, Label) {
    let mut schedule = Schedule::new(Label {});
    schedule.add_systems(consume_system_events).add_systems(
        (escape_on_exit, resize_device, update_frame_time).after(consume_system_events),
    );
    (schedule, Label)
}
