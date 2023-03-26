use bevy_ecs::prelude::{NonSend, NonSendMut, Res};
use winit::window::Window;
use crate::debug_ui::DebugUI;
use crate::state::State;

pub fn update_and_build_debug_ui(
    mut ui: NonSendMut<DebugUI>,
    state: Res<State>,
    window: NonSend<Window>
) {
    ui.update_and_build(&window, state.frame_time.delta, |frame| {
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
                    * Hold right click to control camera\n\
                    * Q-W-E-A-S-D to move\n\
                    * Esc to quit the app");

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
