use bevy_ecs::prelude::*;
use winit::event::*;

use crate::events::{KeyboardEvent, MouseEvent};

// TODO Refactor
#[derive(Resource)]
pub struct Input {
    pub lmb_down: bool,
    pub rmb_down: bool,
    pub w_down: bool,
    pub s_down: bool,
    pub a_down: bool,
    pub d_down: bool,
    pub e_down: bool,
    pub q_down: bool,
    pub esc_down: bool,
    pub space_just_pressed: bool,
    pub tab_just_pressed: bool,
    pub mouse_delta: (f32, f32),

    space_last_pressed: bool,
    tab_last_pressed: bool,
}

impl Input {
    pub fn new() -> Self {
        Input {
            lmb_down: false,
            rmb_down: false,
            w_down: false,
            s_down: false,
            a_down: false,
            d_down: false,
            e_down: false,
            q_down: false,
            esc_down: false,
            mouse_delta: (0.0, 0.0),
            space_just_pressed: false,
            tab_just_pressed: false,
            space_last_pressed: false,
            tab_last_pressed: false,
        }
    }

    pub fn update(
        mut input: ResMut<Input>,
        mut mouse_events: EventReader<MouseEvent>,
        mut keyboard_events: EventReader<KeyboardEvent>,
    ) {
        input.mouse_delta = (0.0, 0.0);
        input.space_just_pressed = false;
        input.tab_just_pressed = false;

        for e in mouse_events.read() {
            match *e {
                MouseEvent::Button { btn, pressed } => match btn {
                    MouseButton::Left => input.lmb_down = pressed,
                    MouseButton::Right => input.rmb_down = pressed,
                    _ => (),
                },

                MouseEvent::Move { dx, dy } => input.mouse_delta = (dx, dy),
            }
        }

        for &KeyboardEvent { code, pressed } in keyboard_events.read() {
            match code {
                VirtualKeyCode::W => input.w_down = pressed,
                VirtualKeyCode::A => input.a_down = pressed,
                VirtualKeyCode::S => input.s_down = pressed,
                VirtualKeyCode::D => input.d_down = pressed,
                VirtualKeyCode::E => input.e_down = pressed,
                VirtualKeyCode::Q => input.q_down = pressed,
                VirtualKeyCode::Escape => input.esc_down = pressed,
                VirtualKeyCode::Space => {
                    input.space_just_pressed = pressed && !input.space_last_pressed;
                    input.space_last_pressed = pressed;
                }
                VirtualKeyCode::Tab => {
                    input.tab_just_pressed = pressed && !input.tab_last_pressed;
                    input.tab_last_pressed = pressed;
                }
                _ => (),
            }
        }
    }
}
