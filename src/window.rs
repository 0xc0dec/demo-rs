use winit::window::CursorGrabMode;

pub trait CursorGrab {
    fn set_cursor_grabbed(&self, grabbed: bool);
}

impl CursorGrab for winit::window::Window {
    fn set_cursor_grabbed(&self, grabbed: bool) {
        if grabbed {
            self.set_cursor_grab(CursorGrabMode::Confined)
                .or_else(|_e| self.set_cursor_grab(CursorGrabMode::Locked))
                .unwrap();
            self.set_cursor_visible(false);
        } else {
            self.set_cursor_grab(CursorGrabMode::None).unwrap();
            self.set_cursor_visible(true);
        }
    }
}
