pub use camera::Camera;
pub use grab::Grab;
pub use physical_body::{PhysicalBody, PhysicalBodyParams};
pub use player::Player;
pub use transform::Transform;

mod camera;
mod grab;
mod physical_body;
mod player;
mod transform;

pub struct RenderTags(pub u32);
pub struct RenderOrder(pub i32);

pub const RENDER_TAG_SCENE: u32 = 0b00000001;
pub const RENDER_TAG_POST_PROCESS: u32 = 0b00000010;
pub const RENDER_TAG_HIDDEN: u32 = 0b00000100;
pub const RENDER_TAG_DEBUG_UI: u32 = 0b00001000;
