mod init;
mod before_update;
mod render;
mod update_physics;
mod escape_on_exit;
mod grab_cursor;

pub use init::init;
pub use before_update::handle_events;
pub use render::render_frame;
pub use update_physics::update_physics;
pub use escape_on_exit::escape_on_exit;
pub use grab_cursor::grab_cursor;