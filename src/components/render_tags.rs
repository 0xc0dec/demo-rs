use bevy_ecs::prelude::Component;

#[derive(Component)]
pub struct RenderTags(pub u32);

pub const RENDER_TAG_SCENE: u32 = 0b00000001;
pub const RENDER_TAG_POST_PROCESS: u32 = 0b00000010;
pub const RENDER_TAG_HIDDEN: u32 = 0b00000100;
pub const RENDER_TAG_DEBUG_UI: u32 = 0b00001000;
