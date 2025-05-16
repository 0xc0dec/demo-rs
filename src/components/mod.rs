pub use camera::Camera;
pub use grab::Grab;
pub use player::Player;
pub use player_target::PlayerTarget;
pub use rigid_body::{RigidBody, RigidBodyParams};
pub use transform::Transform;

use crate::scene::{MaterialHandle, MeshHandle};

mod camera;
mod grab;
mod player;
mod player_target;
mod rigid_body;
mod transform;

pub struct RenderTags(pub u32);
pub struct RenderOrder(pub i32);
pub struct Mesh(pub MeshHandle);
pub struct Material(pub MaterialHandle);

pub const RENDER_TAG_SCENE: u32 = 0b00000001;
pub const RENDER_TAG_POST_PROCESS: u32 = 0b00000010;
pub const RENDER_TAG_HIDDEN: u32 = 0b00000100;
pub const RENDER_TAG_DEBUG_UI: u32 = 0b00001000;
