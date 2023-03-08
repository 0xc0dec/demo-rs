use crate::events::Events;

pub struct FrameContext<'a> {
    pub dt: f32,
    pub events: &'a Events,
}