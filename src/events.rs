use winit::{event::*};
use winit::dpi::PhysicalSize;
use winit::window::{Window, WindowId};

// TODO Make fields public readonly
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
    pub new_surface_size: Option<PhysicalSize<u32>>,
}

impl Events {
    pub fn new(window: &Window) -> Self {
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
            new_surface_size: Some(window.inner_size().into())
        }
    }

    pub fn clear(&mut self) {
        self.mouse_delta = (0.0, 0.0);
        self.new_surface_size = None
    }

    pub fn process_event(&mut self, event: &Event<()>, own_window_id: &WindowId) {
        match event {
            Event::DeviceEvent {
                event: DeviceEvent::MouseMotion { delta, },
                ..
            } => {
                self.mouse_delta = (delta.0 as f32, delta.1 as f32);
            },

            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == own_window_id => {
                match event {
                    WindowEvent::MouseInput {
                        state,
                        button,
                        ..
                    } => {
                        if *button == MouseButton::Right {
                            self.rmb_down = *state == ElementState::Pressed;
                        }
                    }

                    WindowEvent::KeyboardInput {
                        input: KeyboardInput {
                            state: key_state,
                            virtual_keycode: Some(keycode),
                            ..
                        },
                        ..
                    } => {
                        let down = *key_state == ElementState::Pressed;
                        match keycode {
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

                    WindowEvent::Resized(new_size) => {
                        self.new_surface_size = Some(*new_size);
                    },

                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        self.new_surface_size = Some(**new_inner_size);
                    }

                    _ => ()
                }
            }

            _ => ()
        }
    }
}