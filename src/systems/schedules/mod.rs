pub use before_update::new_preupdate_schedule;
pub use render::new_render_schedule;
pub use spawn_scene::new_spawn_scene_schedule;
pub use update::new_update_schedule;

mod before_update;
mod render;
mod spawn_scene;
mod update;
