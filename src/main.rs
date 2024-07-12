use winit::dpi::PhysicalSize;
use winit::event::{DeviceEvent, ElementState, Event, KeyboardInput, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::platform::run_return::EventLoopExtRunReturn;
use winit::window::{Window, WindowBuilder};

use frame_time::FrameTime;
use physics::Physics;
use render::render_pass;

use crate::assets::Assets;
use crate::components::{Camera, Player, RENDER_TAG_DEBUG_UI, RENDER_TAG_POST_PROCESS, Transform};
use crate::debug_ui::DebugUI;
use crate::events::{KeyboardEvent, MouseEvent, ResizeEvent};
use crate::graphics::Graphics;
use crate::input::{Input, InputAction};
use crate::math::Vec3;
use crate::scene::Scene;

mod assets;
mod components;
mod debug_ui;
mod events;
mod frame_time;
mod fs;
mod graphics;
mod input;
mod math;
mod physics;
mod render;
mod scene;

fn consume_system_events(
    event_loop: &mut EventLoop<()>,
    window: &Window,
    debug_ui: &mut DebugUI,
    mouse_events: &mut Vec<MouseEvent>,
    keyboard_events: &mut Vec<KeyboardEvent>,
    resize_event: &mut Option<ResizeEvent>,
) {
    event_loop.run_return(|event, _, flow| {
        *flow = ControlFlow::Poll;

        match event {
            Event::MainEventsCleared => {
                *flow = ControlFlow::Exit;
            }

            Event::DeviceEvent {
                event: DeviceEvent::MouseMotion { delta },
                ..
            } => {
                mouse_events.push(MouseEvent::Move {
                    dx: delta.0 as f32,
                    dy: delta.1 as f32,
                });
            }

            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => match event {
                WindowEvent::MouseInput { state, button, .. } => {
                    mouse_events.push(MouseEvent::Button {
                        btn: *button,
                        pressed: *state == ElementState::Pressed,
                    });
                }

                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            state: key_state,
                            virtual_keycode: Some(keycode),
                            ..
                        },
                    ..
                } => {
                    keyboard_events.push(KeyboardEvent {
                        code: *keycode,
                        pressed: *key_state == ElementState::Pressed,
                    });
                }

                WindowEvent::Resized(new_size) => {
                    resize_event.replace(ResizeEvent(*new_size));
                }

                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    resize_event.replace(ResizeEvent(**new_inner_size));
                }

                _ => (),
            },

            _ => {}
        }

        // TODO Make DebugUI consume events from the event vectors we're filling in this function.
        debug_ui.handle_window_event(window, &event);
    });
}

fn build_debug_ui(ui: &mut DebugUI, frame_time: &FrameTime, window: &Window) {
    ui.prepare_render(window, frame_time.delta, |frame| {
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
                    "Controls:\n\
                    - Toggle camera control: Tab\n\
                    - Move: WASDQE\n\
                    - Grab objects: hold LMB\n\
                    - Spawn new box: Space\n\
                    - Quit: Escape",
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
    });
}

// TODO Update deps
// TODO Grabbing objects with a cursor (when camera is not controlled)

fn main() {
    let mut event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Demo")
        .with_inner_size(PhysicalSize {
            width: 1900,
            height: 1200,
        })
        .build(&event_loop)
        .unwrap();
    // Store device + window in a new struct Device (or smth like that), add Deref traits to it.
    let mut gfx = pollster::block_on(Graphics::new(&window));
    let mut physics = Physics::new();
    let mut input = Input::new();
    let mut frame_time = FrameTime::new();

    let assets = Assets::load(&gfx);
    let mut debug_ui = DebugUI::new(&gfx, &window);

    // TODO More optimal, avoid vec cleanup on each iteration
    let mut mouse_events = Vec::new();
    let mut keyboard_events = Vec::new();
    let mut resize_event = None;

    // TODO Replace with a proper ECS or restructure in some other better way
    let mut scene = Scene::new();

    // Player is outside the normal components set for convenience because it's a singleton.
    // Ideally it should be unified with the rest of the objects once we have a proper ECS
    // or an alternative.
    let mut player = Player::new(&gfx, &mut physics);
    let player_target_idx = scene.spawn_player_target(&gfx, &assets);

    let pp_cam = Camera::new(1.0, RENDER_TAG_POST_PROCESS | RENDER_TAG_DEBUG_UI, None);

    scene.spawn_floor(&gfx, &assets, &mut physics);
    // Spawning skybox last to ensure the sorting by render order works and it still shows up
    // in the background.
    scene.spawn_skybox(&gfx, &assets);
    let pp_idx = scene.spawn_post_process_overlay(
        player.camera().target().as_ref().unwrap().color_tex(),
        &gfx,
        &assets,
    );

    let mut spawned_demo_box = false;

    while !input.action_active(InputAction::Escape) {
        consume_system_events(
            &mut event_loop,
            &window,
            &mut debug_ui,
            &mut mouse_events,
            &mut keyboard_events,
            &mut resize_event,
        );

        if let Some(e) = resize_event.as_ref() {
            gfx.resize(e.0);
        }

        input.update(&mouse_events, &keyboard_events);
        frame_time = frame_time.advance();
        physics.update(frame_time.delta);

        player.update(
            &gfx,
            &frame_time,
            &input,
            &window,
            &mut physics,
            &resize_event,
        );

        if resize_event.is_some() {
            scene.update_post_process_overlay(
                pp_idx,
                player.camera().target().as_ref().unwrap().color_tex(),
                &gfx,
                &assets,
            );
        }

        scene.update_grabbed(&player, &input, &mut physics);
        scene.update_player_target(&player, player_target_idx);

        scene.sync_physics(&physics);

        // Spawn a single box automatically
        {
            if input.action_activated(InputAction::Spawn) || !spawned_demo_box {
                let pos = if spawned_demo_box {
                    player.transform().position() + player.transform().forward().xyz() * 5.0
                } else {
                    spawned_demo_box = true;
                    Vec3::y_axis().xyz() * 5.0
                };
                scene.spawn_cube(pos, Vec3::from_element(1.0), &gfx, &assets, &mut physics);
            }
        }

        // Render main scene into a texture
        render_pass(
            &gfx,
            &scene.build_render_bundles(player.camera(), player.transform(), &gfx),
            player.camera().target().as_ref(),
            None,
        );

        // Render post-process overlay + debug UI
        build_debug_ui(&mut debug_ui, &frame_time, &window);
        render_pass(
            &gfx,
            &scene.build_render_bundles(&pp_cam, &Transform::default(), &gfx),
            None,
            Some(&mut debug_ui),
        );

        mouse_events.clear();
        keyboard_events.clear();
        resize_event.take();
    }
}
