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

use bevy_ecs::prelude::{IntoSystemConfig, run_once, Schedule, World};
use crate::components::{Player, Skybox};
use crate::debug_ui::DebugUI;
use crate::state::State;
use crate::systems::{before_update, init, render_frame};

fn main() {
    let mut world = World::default();

    Schedule::default()
        .add_system(init)
        .add_system(Player::spawn.after(init))
        .add_system(Skybox::spawn.after(init))
        .run(&mut world);


    let mut preupdate_schedule = Schedule::default();
    preupdate_schedule.add_system(before_update);

    let mut update_schedule = Schedule::default();
    update_schedule
        .add_system(Player::update)
        .add_system(DebugUI::update);

    let mut render_schedule = Schedule::default();
    render_schedule.add_system(render_frame);

    // Could have used a single schedule but it seems easier for now to use separate

    loop {
        preupdate_schedule.run(&mut world);
        update_schedule.run(&mut world);
        render_schedule.run(&mut world);

        if !world.get_resource::<State>().unwrap().running { return; }

        // scene.update(&frame_context);
        // debug_ui.update(&frame_context);
        //
        // {
        //     let mut frame = app.device.new_frame(Some(pp.source_rt()));
        //     scene.render(&mut frame, &frame_context);
        //     frame.finish(None);
        // }
        //
        // {
        //     let mut frame = app.device.new_frame(None);
        //     pp.render(&mut frame);
        //     debug_ui.build_frame(&app.window, |frame| scene.build_debug_ui(frame));
        //     frame.finish(Some(&mut debug_ui));
        // }
    }
}
