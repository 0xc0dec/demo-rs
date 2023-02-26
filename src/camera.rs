use cgmath::InnerSpace;
use winit::event::{MouseButton, VirtualKeyCode};

pub struct Camera {
    eye: cgmath::Point3<f32>,
    target: cgmath::Point3<f32>,
    dir: cgmath::Vector3<f32>,
    up: cgmath::Vector3<f32>,
    aspect: f32,
    fovy: f32,
    znear: f32,
    zfar: f32,
}

impl Camera {
    pub fn new(eye: cgmath::Point3<f32>, target: cgmath::Point3<f32>, canvas_width: f32, canvas_height: f32) -> Self {
        Self {
            eye,
            target,
            dir: (target - eye).normalize(),
            up: cgmath::Vector3::unit_y(),
            aspect: canvas_width / canvas_height,
            fovy: 45.0,
            znear: 0.1,
            zfar: 100.0,
        }
    }

    pub fn view_proj_matrix(&self) -> cgmath::Matrix4<f32> {
        let view = cgmath::Matrix4::look_to_rh(self.eye, self.dir, self.up);
        let proj = cgmath::perspective(cgmath::Deg(self.fovy), self.aspect, self.znear, self.zfar);
        return proj * view;
    }
}

pub struct CameraController {
    speed: f32,
    forward_pressed: bool,
    backward_pressed: bool,
    left_pressed: bool,
    right_pressed: bool,
    lmb_down: bool,
}

impl CameraController {
    pub fn new(speed: f32) -> Self {
        Self {
            speed,
            forward_pressed: false,
            backward_pressed: false,
            left_pressed: false,
            right_pressed: false,
            lmb_down: false
        }
    }

    pub fn process_mouse_movement(&self, _delta: (f64, f64), _time_delta: f32) {
        if !self.lmb_down { return }

        // TODO
    }

    pub fn process_mouse_button(&mut self, button: MouseButton, down: bool) {
        if button == MouseButton::Left {
            self.lmb_down = down;
        }
    }

    pub fn process_keyboard(&mut self, keycode: VirtualKeyCode, pressed: bool) {
        match keycode {
            VirtualKeyCode::W | VirtualKeyCode::Up => {
                self.forward_pressed = pressed;
            }
            VirtualKeyCode::A | VirtualKeyCode::Left => {
                self.left_pressed = pressed;
            }
            VirtualKeyCode::S | VirtualKeyCode::Down => {
                self.backward_pressed = pressed;
            }
            VirtualKeyCode::D | VirtualKeyCode::Right => {
                self.right_pressed = pressed;
            }
            _ => (),
        }
    }

    pub fn update_camera(&self, camera: &mut Camera) {
        let forward = camera.target - camera.eye;
        let forward_norm = forward.normalize();
        let forward_mag = forward.magnitude();

        // Prevents glitching when camera gets too close to the
        // center of the scene.
        if self.forward_pressed && forward_mag > self.speed {
            camera.eye += forward_norm * self.speed;
        }
        if self.backward_pressed {
            camera.eye -= forward_norm * self.speed;
        }

        let right = forward_norm.cross(camera.up);

        // Redo radius calc in case the forward/backward is pressed.
        let forward = camera.target - camera.eye;
        let forward_mag = forward.magnitude();

        if self.right_pressed {
            // Rescale the distance between the target and eye so
            // that it doesn't change. The eye therefore still
            // lies on the circle made by the target and eye.
            camera.eye = camera.target - (forward + right * self.speed).normalize() * forward_mag;
        }
        if self.left_pressed {
            camera.eye = camera.target - (forward - right * self.speed).normalize() * forward_mag;
        }

        camera.dir = (camera.target - camera.eye).normalize();
    }
}