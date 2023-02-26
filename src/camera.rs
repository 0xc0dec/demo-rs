use cgmath::{InnerSpace, Matrix4, Rad, Vector3, Zero};
use winit::event::{MouseButton, VirtualKeyCode};
use crate::transform::{Transform, TransformSpace};

pub struct Camera {
    aspect: f32,
    fovy: f32,
    znear: f32,
    zfar: f32,
    forward_pressed: bool,
    backward_pressed: bool,
    left_pressed: bool,
    right_pressed: bool,
    up_pressed: bool,
    down_pressed: bool,
    rmb_down: bool,
    transform: Transform,
    mouse_delta_x: f32,
    mouse_delta_y: f32,
}

impl Camera {
    pub fn new(eye: Vector3<f32>, target: Vector3<f32>, canvas_width: f32, canvas_height: f32) -> Self {
        let mut cam = Self {
            aspect: canvas_width / canvas_height,
            fovy: 45.0,
            znear: 0.1,
            zfar: 100.0,
            transform: Transform::new(),
            forward_pressed: false,
            backward_pressed: false,
            left_pressed: false,
            right_pressed: false,
            up_pressed: false,
            down_pressed: false,
            rmb_down: false,
            mouse_delta_x: 0.0,
            mouse_delta_y: 0.0
        };
        cam.transform.look_at(eye, target);
        cam
    }

    pub fn view_proj_matrix(&self) -> Matrix4<f32> {
        let proj = cgmath::perspective(cgmath::Deg(self.fovy), self.aspect, self.znear, self.zfar);
        return proj * self.transform.matrix();
    }

    pub fn process_mouse_movement(&mut self, delta: (f64, f64)) {
        self.mouse_delta_x = delta.0 as f32;
        self.mouse_delta_y = delta.1 as f32;
    }

    pub fn process_mouse_button(&mut self, button: MouseButton, down: bool) {
        if button == MouseButton::Right {
            self.rmb_down = down;
        }
    }

    pub fn process_keyboard(&mut self, keycode: VirtualKeyCode, pressed: bool) {
        match keycode {
            VirtualKeyCode::W => self.forward_pressed = pressed,
            VirtualKeyCode::A => self.left_pressed = pressed,
            VirtualKeyCode::S => self.backward_pressed = pressed,
            VirtualKeyCode::D => self.right_pressed = pressed,
            VirtualKeyCode::E => self.up_pressed = pressed,
            VirtualKeyCode::Q => self.down_pressed = pressed,
            _ => (),
        }
    }

    pub fn update(&mut self, dt: f32) {
        if self.rmb_down {
            let horiz_angle = Rad(self.mouse_delta_x as f32 * dt) * 1.0;
            self.transform.rotate_around_axis(Vector3::unit_y(), horiz_angle, TransformSpace::World);

            let vert_angle = Rad(self.mouse_delta_y as f32 * dt) * 1.0;
            self.transform.rotate_around_axis(Vector3::unit_x(), vert_angle, TransformSpace::Local);
        }

        let mut movement: Vector3<f32> = Vector3::zero();
        if self.forward_pressed {
            movement += Vector3::unit_z();
        }
        if self.backward_pressed {
            movement -= Vector3::unit_z();
        }
        if self.right_pressed {
            movement -= Vector3::unit_x();
        }
        if self.left_pressed {
            movement += Vector3::unit_x();
        }
        if self.up_pressed {
            movement -= Vector3::unit_y();
        }
        if self.down_pressed {
            movement += Vector3::unit_y();
        }
        if !movement.is_zero() {
            self.transform.translate(movement.normalize() * dt * 10.0);
        }

        self.mouse_delta_x = 0.0;
        self.mouse_delta_y = 0.0;
    }
}
