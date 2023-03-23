mod camera;
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
mod scene2;
mod state;
mod systems;

use bevy_ecs::prelude::{IntoSystemConfig, Schedule, World, run_once};

use crate::scene2::Scene2;
use crate::state::State;
use crate::systems::{before_update, init};

fn main() {
    let mut world = World::default();
    let mut schedule = Schedule::default();
    schedule
        .add_system(init.run_if(run_once()))
        .add_system(Scene2::init
            .after(init)
            .run_if(run_once())
        )
        .add_system(before_update.after(Scene2::init));
    // Scene2::configure_update_systems(&mut schedule);

    // let mut update = Schedule::default();
    // // TODO Ensure before_update always runs before all other updates
    // update.add_system(before_update);
    // Scene2::configure_update_systems(&mut update);

    loop {
        schedule.run(&mut world);

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
