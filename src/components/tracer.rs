use bevy_ecs::prelude::{Commands, Component, NonSend};
use crate::device::Device;

#[derive(Component)]
pub struct Tracer;

impl Tracer {
    pub fn spawn(mut commands: Commands, device: NonSend<Device>) {
        commands.spawn((Tracer, ));
    }
}