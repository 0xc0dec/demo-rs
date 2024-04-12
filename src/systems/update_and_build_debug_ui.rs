use bevy_ecs::prelude::*;
use winit::window::Window;

use crate::components::Camera;
use crate::debug_ui::DebugUI;
use crate::render_tags::RENDER_TAG_DEBUG_UI;
use crate::resources::FrameTime;

pub fn update_and_build_debug_ui(
    mut ui: NonSendMut<DebugUI>,
    cameras: Query<&Camera>,
    frame_time: Res<FrameTime>,
    window: NonSend<Window>,
) {
    if !cameras.iter().any(|c| c.should_render(RENDER_TAG_DEBUG_UI)) {
        // No point. Also if we don't render the UI later (because there's no cameras)
        // imgui will fail on the next iteration when we start a new frame because the previous frame
        // will not been finished (i.e. rendered). Perhaps that could be worked around differently.
        return;
    }

    ui.prepare_render(&window, frame_time.delta, |frame| {
        frame
            .window("Debug info")
            .position([10.0, 10.0], imgui::Condition::FirstUseEver)
            .movable(false)
            .resizable(false)
            .always_auto_resize(true)
            .collapsible(false)
            .no_decoration()
            .build(|| {
                frame.text(
                    "\
                    Controls:\n\
                    * Control camera: hold RMB\n\
                    * Move (while controlling camera): WASDQE\n\
                    * Grab and release objects: LMB\n\
                    * Spawn new box: Space\n\
                    * Quit: Escape",
                );

                let mut mouse_pos = frame.io().mouse_pos;
                // Prevent UI jumping at start when the mouse position is not yet known
                // and imgui returns extra huge numbers.
                if !(-10000.0f32..10000.0f32).contains(&mouse_pos[0]) {
                    mouse_pos = [-1.0f32, -1.0f32];
                }
                frame.text(format!(
                    "Mouse position: ({:.1},{:.1})",
                    mouse_pos[0], mouse_pos[1]
                ));
            });
    })
}
