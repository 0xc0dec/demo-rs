use bevy_ecs::prelude::*;
use winit::event::*;

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
    // TODO Fix, it triggers every frame while the key is pressed
    pub space_just_pressed: bool,
    // TODO Fix, same as above
    pub tab_just_pressed: bool,
    pub mouse_delta: (f32, f32),
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
            space_just_pressed: false,
            tab_just_pressed: false,
            mouse_delta: (0.0, 0.0),
        }
    }

    pub fn reset(&mut self) {
        self.mouse_delta = (0.0, 0.0);
        self.space_just_pressed = false;
        self.tab_just_pressed = false;
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
            VirtualKeyCode::Space => self.space_just_pressed = pressed,
            VirtualKeyCode::Tab => self.tab_just_pressed = pressed,
            _ => (),
        }
    }
}
