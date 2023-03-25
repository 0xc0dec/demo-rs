use bevy_ecs::prelude::{NonSend, ResMut};
use crate::input::Input;
use crate::state::State;

pub fn escape_on_exit(input: NonSend<Input>, mut state: ResMut<State>) {
    if input.escape_down {
        state.running = false;
    }
}