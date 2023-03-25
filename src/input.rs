use winit::event::*;

// TODO Move into State, rename to InputState
// TODO Mark as resource
pub struct Input {
    pub rmb_down: bool,
    pub rmb_down_just_switched: bool,
    pub forward_down: bool,
    pub back_down: bool,
    pub left_down: bool,
    pub right_down: bool,
    pub up_down: bool,
    pub down_down: bool,
    pub mouse_delta: (f32, f32),
}

impl Input {
    pub fn new() -> Self {
        Input {
            rmb_down: false,
            rmb_down_just_switched: false,
            forward_down: false,
            back_down: false,
            left_down: false,
            right_down: false,
            up_down: false,
            down_down: false,
            mouse_delta: (0.0, 0.0),
        }
    }

    pub fn reset(&mut self) {
        self.mouse_delta = (0.0, 0.0);
        self.rmb_down_just_switched = false;
    }

    pub fn on_mouse_move(&mut self, delta: (f32, f32)) {
        self.mouse_delta = delta;
    }

    pub fn on_mouse_button(&mut self, btn: MouseButton, pressed: bool) {
        if btn == MouseButton::Right {
            self.rmb_down = pressed;
            self.rmb_down_just_switched = true;
        }
    }

    pub fn on_key(&mut self, code: VirtualKeyCode, pressed: bool) {
        match code {
            VirtualKeyCode::W => self.forward_down = pressed,
            VirtualKeyCode::A => self.left_down = pressed,
            VirtualKeyCode::S => self.back_down = pressed,
            VirtualKeyCode::D => self.right_down = pressed,
            VirtualKeyCode::E => self.up_down = pressed,
            VirtualKeyCode::Q => self.down_down = pressed,
            _ => (),
        }
    }
}
