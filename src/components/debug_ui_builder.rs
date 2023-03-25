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
                    let mouse_pos = frame.io().mouse_pos;
                    frame.text(format!(
                        "Mouse position: ({:.1},{:.1})",
                        mouse_pos[0], mouse_pos[1]
                    ));
                });
        })
    }
}
