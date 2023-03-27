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
        .add_system(Skybox::spawn.after(init))
        .add_system(FloorBox::spawn.after(init))
        .add_system(FreeBox::spawn.after(init))
        .add_system(Player::spawn.after(init))
        .add_system(Tracer::spawn.after(init))
        .run(&mut world);

    // PP requires that Player be already spawned and we cannot guarantee that so we're using
    // this hack. Should be done better with some systems magic.
    Schedule::default()
        .add_system(PostProcessor::spawn)
        .run(&mut world);

    let mut preupdate_schedule = Schedule::default();
    preupdate_schedule
        .add_system(handle_system_events)
        .add_system(escape_on_exit.after(handle_system_events))
        .add_system(grab_cursor.after(handle_system_events))
        .add_system(resize_device.after(handle_system_events))
        .add_system(update_input_state.after(handle_system_events))
        .add_system(update_frame_time.after(handle_system_events));

    let mut update_schedule = Schedule::default();
    update_schedule
        // TOO Run physics last?
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
