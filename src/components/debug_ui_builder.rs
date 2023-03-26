use bevy_ecs::prelude::{Component, NonSend, NonSendMut};
use winit::window::Window;
use crate::debug_ui::DebugUI;

#[derive(Component)]
pub struct DebugUIBuilder;

impl DebugUIBuilder {
    pub fn build_debug_ui(mut debug_ui: NonSendMut<DebugUI>, window: NonSend<Window>) {
        debug_ui.build(&window, |frame| {
            frame
                .window("Debug info")
                .position([10.0, 10.0], imgui::Condition::FirstUseEver)
                .movable(false)
                .resizable(false)
                .always_auto_resize(true)
                .collapsible(false)
                .no_decoration()
                .build(|| {
                    frame.text("\
                    Controls:\n\
                      \tHold right click to control camera\n\
                      \tQ-W-E-A-S-D to move\n\
                      \tEsc to quit the app
                    ");

                    let mut mouse_pos = frame.io().mouse_pos;
                    // Prevent UI jumping at start when the mouse position is not yet known
                    // and imgui returns extra huge numbers.
                    if !(-10000.0f32..10000.0f32).contains(&mouse_pos[0]) {
                        mouse_pos = [0.0f32, 0.0f32];
                    }
                    frame.text(format!(
                        "Mouse position: ({:.1},{:.1})",
                        mouse_pos[0], mouse_pos[1]
                    ));
                });
        })
    }
}
