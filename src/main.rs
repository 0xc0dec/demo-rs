mod debug_ui;
mod device;
mod input_state;
mod math;
mod model;
mod physics_world;
mod post_processor;
mod render_target;
mod assets;
mod shaders;
mod texture;
mod transform;
mod frame_time;
mod state;
mod systems;
mod components;
mod events;

use bevy_ecs::prelude::*;
use crate::components::*;
use crate::debug_ui::DebugUI;
use crate::state::State;
use crate::systems::*;

fn main() {
    let mut world = World::default();

    // TODO Try to use less schedules by adding more complex rules

    Schedule::default()
        .add_system(init)
        .add_system(Player::spawn.after(init))
        .add_system(Skybox::spawn.after(init))
        .add_system(FloorBox::spawn.after(init))
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
        .add_system(Player::update.after(update_physics))
        .add_system(DebugUI::update.after(update_physics))
        .add_system(DebugUIBuilder::build_debug_ui.after(DebugUI::update));

    let mut render_schedule = Schedule::default();
    render_schedule.add_system(render_frame);

    loop {
        preupdate_schedule.run(&mut world);
        update_schedule.run(&mut world);
        render_schedule.run(&mut world);

        if !world.resource::<State>().running { return; }

        // {
        //     let mut frame = app.device.new_frame(None);
        //     pp.render(&mut frame);
        //     debug_ui.build_frame(&app.window, |frame| scene.build_debug_ui(frame));
        //     frame.finish(Some(&mut debug_ui));
        // }
    }
}
