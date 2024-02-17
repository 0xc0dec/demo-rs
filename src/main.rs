use bevy_ecs::prelude::*;

use crate::app::App;
use crate::assets::Assets;
use crate::systems::*;

mod app;
mod assets;
mod components;
mod debug_ui;
mod device;
mod events;
mod frame_time;
mod input;
mod math;
mod mesh;
mod physics_world;
mod render_tags;
mod render_target;
mod shaders;
mod systems;
mod texture;

fn main() {
    let mut world = World::default();
    world.init_resource::<Schedules>();

    Schedule::default()
        .add_systems((init_app, Assets::load.after(init_app)))
        .run(&mut world);

    let spawn_scene = new_spawn_scene_schedule();
    world.add_schedule(spawn_scene.0);

    let before_update = new_before_update_schedule();
    world.add_schedule(before_update.0);

    let update = new_update_schedule();
    world.add_schedule(update.0);

    let render = new_render_schedule();
    world.add_schedule(render.0);

    loop {
        world.run_schedule(spawn_scene.1);
        world.run_schedule(before_update.1);
        world.run_schedule(update.1);
        world.run_schedule(render.1);

        if !world.resource::<App>().running {
            break;
        }
    }
}
