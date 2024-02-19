use bevy_ecs::prelude::*;

use crate::resources::*;
use crate::systems::*;

mod assets;
mod components;
mod debug_ui;
mod math;
mod render_tags;
mod render_target;
mod resources;
mod systems;

// TODO:
// - Use wgpu types with `wgpu::` prefix.
// - Remove `::*` imports.
// - Refactor debug UI rendering, it should be a component, the `render()` system should now render it explicitly.
// - Load meshes within `Assets`.
// - Transform hierarchies.

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
