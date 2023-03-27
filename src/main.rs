mod assets;
mod components;
mod debug_ui;
mod device;
mod events;
mod frame_time;
mod input_state;
mod math;
mod model;
mod physics_world;
mod render_tags;
mod render_target;
mod shaders;
mod state;
mod systems;
mod texture;

use crate::components::*;
use crate::state::State;
use crate::systems::*;
use bevy_ecs::prelude::*;

fn main() {
    let mut world = World::default();

    // TODO Try to use less schedules by adding more complex rules

    Schedule::default()
        .add_system(init)
        .add_systems((
            Skybox::spawn,
            FloorBox::spawn,
            FreeBox::spawn,
            Player::spawn,
            Tracer::spawn
        ).after(init))
        .run(&mut world);

    // PP requires that Player be already spawned and we cannot guarantee that so we're using
    // this hack. Should be done better with some systems magic.
    Schedule::default()
        .add_system(PostProcessor::spawn)
        .run(&mut world);

    let mut preupdate_schedule = Schedule::default();
    preupdate_schedule
        .add_system(handle_system_events)
        .add_systems((
            escape_on_exit,
            grab_cursor,
            resize_device,
            update_input_state,
            update_frame_time,
        ).after(handle_system_events));

    let mut update_schedule = Schedule::default();
    update_schedule
        .add_system(update_physics)
        .add_system(PhysicsBody::sync.after(update_physics))
        .add_system(Player::update.after(update_physics))
        .add_system(Tracer::update.after(Player::update))
        .add_system(update_and_build_debug_ui.after(update_physics));

    let mut render_schedule = Schedule::default();
    render_schedule.add_system(render);

    loop {
        preupdate_schedule.run(&mut world);
        update_schedule.run(&mut world);
        render_schedule.run(&mut world);

        if !world.resource::<State>().running {
            return;
        }
    }
}
