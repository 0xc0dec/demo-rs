use winit::dpi::PhysicalSize;
use winit::event::{DeviceEvent, ElementState, Event, KeyEvent, WindowEvent};
use winit::event_loop::EventLoop;
use winit::keyboard::PhysicalKey::Code;
use winit::platform::run_on_demand::EventLoopExtRunOnDemand;
use winit::window::Window;

use frame_time::FrameTime;
use physics::Physics;
use render::render_pass;

use crate::assets::Assets;
use crate::camera::Camera;
use crate::events::{KeyboardEvent, MouseEvent, ResizeEvent};
use crate::graphics::Graphics;
use crate::input::{Input, InputAction};
use crate::math::Vec3;
use crate::player::Player;
use crate::render_tags::{RENDER_TAG_DEBUG_UI, RENDER_TAG_POST_PROCESS};
use crate::scene::Scene;
use crate::transform::Transform;

mod assets;
mod camera;
mod events;
mod frame_time;
mod fs;
mod graphics;
mod input;
mod materials;
mod math;
mod mesh;
mod physical_body;
mod physics;
mod player;
mod render;
mod render_tags;
mod render_target;
mod scene;
mod texture;
mod transform;

fn consume_system_events(
    event_loop: &mut EventLoop<()>,
    window: &Window,
    mouse_events: &mut Vec<MouseEvent>,
    keyboard_events: &mut Vec<KeyboardEvent>,
    resize_event: &mut Option<ResizeEvent>,
) {
    let _ = event_loop.run_on_demand(|event, target| match event {
        Event::AboutToWait => target.exit(),
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
                event:
                    KeyEvent {
                        state: key_state,
                        physical_key: key,
                        ..
                    },
                ..
            } => {
                if let Code(code) = key {
                    keyboard_events.push(KeyboardEvent {
                        code: *code,
                        pressed: *key_state == ElementState::Pressed,
                    });
                }
            }

            WindowEvent::Resized(new_size) => {
                resize_event.replace(ResizeEvent(*new_size));
            }

            _ => (),
        },

        _ => {}
    });
}

// TODO egui (https://github.com/ejb004/egui-wgpu-demo) or other UI. Currently they all seem unusable after the recent
// update of winit to versions 0.29 - 0.30
// TODO Switch to what winit recommends instead of the deprecated stuff.
// TODO Grabbing objects with a cursor (when camera is not controlled).

fn main() {
    let mut event_loop = EventLoop::new().unwrap();
    let window = event_loop
        .create_window(
            Window::default_attributes()
                .with_title("Demo")
                .with_inner_size(PhysicalSize {
                    width: 1900,
                    height: 1200,
                }),
        )
        .unwrap();
    // Store device + window in a new struct Device (or smth like that), add Deref traits to it.
    let mut gfx = pollster::block_on(Graphics::new(&window));
    let mut physics = Physics::new();
    let mut input = Input::new();
    let mut frame_time = FrameTime::new();

    let assets = Assets::load(&gfx);

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
        );

        // Render post-process overlay
        render_pass(
            &gfx,
            &scene.build_render_bundles(&pp_cam, &Transform::default(), &gfx),
            None,
        );

        mouse_events.clear();
        keyboard_events.clear();
        resize_event.take();
    }
}
