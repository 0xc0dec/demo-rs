use bevy_ecs::prelude::EventReader;
use bevy_ecs::system::NonSendMut;
use crate::device::{Device, SurfaceSize};
use crate::events::WindowResized;

pub fn resize_device(
    mut device: NonSendMut<Device>,
    mut events: EventReader<WindowResized>
) {
    for evt in events.iter() {
        device.resize(evt.new_size);
    }
}