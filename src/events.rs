use winit::event::*;

pub struct Events {
    pub rmb_down: bool,
    pub forward_down: bool,
    pub back_down: bool,
    pub left_down: bool,
    pub right_down: bool,
    pub up_down: bool,
    pub down_down: bool,
    pub escape_down: bool,
    pub mouse_delta: (f32, f32),
}

impl Events {
    pub fn new() -> Self {
        Events {
            rmb_down: false,
            forward_down: false,
            back_down: false,
            left_down: false,
            right_down: false,
            up_down: false,
            down_down: false,
            escape_down: false,
            mouse_delta: (0.0, 0.0),
        }
    }

    pub fn reset(&mut self) {
        self.mouse_delta = (0.0, 0.0);
    }

    pub fn on_mouse_move(&mut self, delta: (f32, f32)) {
        self.mouse_delta = delta;
    }

    pub fn on_mouse_button(&mut self, btn: &MouseButton, state: &ElementState) {
        if *btn == MouseButton::Right {
            self.rmb_down = *state == ElementState::Pressed;
        }
    }

    pub fn on_key(&mut self, code: &VirtualKeyCode, state: &ElementState) {
        let down = *state == ElementState::Pressed;
        match code {
            VirtualKeyCode::W => self.forward_down = down,
            VirtualKeyCode::A => self.left_down = down,
            VirtualKeyCode::S => self.back_down = down,
            VirtualKeyCode::D => self.right_down = down,
            VirtualKeyCode::E => self.up_down = down,
            VirtualKeyCode::Q => self.down_down = down,
            VirtualKeyCode::Escape => self.escape_down = down,
            _ => ()
        }
    }
}