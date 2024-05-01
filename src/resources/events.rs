use bevy_ecs::prelude::*;
use winit::event::*;

use crate::resources::SurfaceSize;

// TODO Refactor
#[derive(Resource)]
pub struct Events {
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
    pub new_surface_size: Option<SurfaceSize>,

    space_last_pressed: bool,
    tab_last_pressed: bool,
}

impl Events {
    pub fn new() -> Self {
        Events {
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
            new_surface_size: None,
            space_last_pressed: false,
            tab_last_pressed: false,
        }
    }

    pub fn reset(&mut self) {
        self.mouse_delta = (0.0, 0.0);
        self.space_just_pressed = false;
        self.tab_just_pressed = false;
        self.new_surface_size = None;
    }

    pub fn on_mouse_move(&mut self, delta: (f32, f32)) {
        self.mouse_delta = delta;
    }

    pub fn on_mouse_button(&mut self, btn: MouseButton, pressed: bool) {
        if btn == MouseButton::Left {
            self.lmb_down = pressed;
        }
        if btn == MouseButton::Right {
            self.rmb_down = pressed;
        }
    }

    pub fn on_key(&mut self, code: VirtualKeyCode, pressed: bool) {
        match code {
            VirtualKeyCode::W => self.w_down = pressed,
            VirtualKeyCode::A => self.a_down = pressed,
            VirtualKeyCode::S => self.s_down = pressed,
            VirtualKeyCode::D => self.d_down = pressed,
            VirtualKeyCode::E => self.e_down = pressed,
            VirtualKeyCode::Q => self.q_down = pressed,
            VirtualKeyCode::Escape => self.esc_down = pressed,
            VirtualKeyCode::Space => {
                self.space_just_pressed = pressed && !self.space_last_pressed;
                self.space_last_pressed = pressed;
            }
            VirtualKeyCode::Tab => {
                self.tab_just_pressed = pressed && !self.tab_last_pressed;
                self.tab_last_pressed = pressed;
            }
            _ => (),
        }
    }
}
