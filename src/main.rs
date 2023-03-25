mod debug_ui;
mod device;
mod frame_context;
mod input;
mod math;
mod model;
mod physics_world;
mod post_processor;
mod render_target;
mod assets;
mod scene;
mod shaders;
mod texture;
mod transform;
mod app;
mod frame_time;
mod state;
mod systems;
mod components;

use bevy_ecs::prelude::*;
use crate::components::*;
use crate::debug_ui::DebugUI;
use crate::math::Vec3;
use crate::state::State;
use crate::systems::*;

fn main() {
    let mut world = World::default();

    // TODO Try to use less schedules by adding more complex rules

    Schedule::default()
        .add_system(init)
        .add_system(Player::spawn.after(init))
        .add_system(Skybox::spawn.after(init))
        .add_system(PhysicsBox::spawn(
            PhysicsBoxParams {
                pos: Vec3::from_element(0.0),
                scale: Vec3::new(10.0, 0.5, 10.0),
                rotation_axis: Vec3::from_element(0.0),
                rotation_angle: 0.0,
                movable: false,
            }
        ).after(init))
        .run(&mut world);

    let mut preupdate_schedule = Schedule::default();
    preupdate_schedule
        .add_system(before_update)
        .add_system(escape_on_exit.after(before_update))
        .add_system(grab_cursor.after(before_update));

    let mut update_schedule = Schedule::default();
    update_schedule
        // TOO Run physics last?
        .add_system(update_physics)
        .add_system(Player::update.after(update_physics))
        .add_system(DebugUI::update.after(update_physics));

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
