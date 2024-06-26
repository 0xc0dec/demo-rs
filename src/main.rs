use bevy_ecs::prelude::*;
use bevy_ecs::system::RunSystemOnce;

use crate::resources::*;
use crate::systems::*;

mod assets;
mod components;
mod debug_ui;
mod events;
mod math;
mod resources;
mod systems;

fn main() {
    let mut world = World::default();
    world.init_resource::<Schedules>();

    world.run_system_once(init);
    world.run_system_once(Assets::load);

    let spawn_scene = new_spawn_scene_schedule();
    world.add_schedule(spawn_scene.0);

    let before_update = new_before_update_schedule();
    world.add_schedule(before_update.0);

    let update = new_update_schedule();
    world.add_schedule(update.0);

    loop {
        world.run_schedule(spawn_scene.1);
        world.run_schedule(before_update.1);
        world.run_schedule(update.1);
        world.run_system_once(render);

        if !world.resource::<App>().running {
            break;
        }
    }
}
