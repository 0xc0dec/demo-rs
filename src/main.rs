use std::sync::Arc;

use winit::application::ApplicationHandler;
use winit::dpi::PhysicalSize;
use winit::event::{DeviceEvent, DeviceId, ElementState, KeyEvent, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::PhysicalKey::Code;
use winit::window::{Window, WindowId};

use crate::assets::Assets;
use crate::camera::Camera;
use crate::frame_time::FrameTime;
use crate::graphics::{Graphics, SurfaceSize};
use crate::input::{Input, InputAction};
use crate::math::Vec3;
use crate::physics::Physics;
use crate::player::Player;
use crate::render::render_pass;
use crate::render_tags::{RENDER_TAG_DEBUG_UI, RENDER_TAG_POST_PROCESS};
use crate::scene::Scene;
use crate::transform::Transform;

mod assets;
mod camera;
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

// TODO egui (https://github.com/ejb004/egui-wgpu-demo) or other UI. Currently they all seem unusable after the recent
// update of winit to versions 0.29 - 0.30
// TODO Grabbing objects with a cursor (when camera is not controlled).

#[derive(Default)]
struct State<'a> {
    window: Option<Arc<Window>>,
    gfx: Option<Graphics<'a>>,
    assets: Option<Assets>,
    scene: Option<Scene>,
    player: Option<Player>,
    pp_cam: Option<Camera>,
    pp_id: usize,
    player_target_id: usize,
    input: Option<Input>,
    physics: Option<Physics>,
    frame_time: Option<FrameTime>,
    spawned_demo_box: bool,
    new_canvas_size: Option<SurfaceSize>,
}

impl<'a> ApplicationHandler for State<'a> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }

        let window = Arc::new(
            event_loop
                .create_window(
                    Window::default_attributes()
                        .with_inner_size(PhysicalSize {
                            width: 1900,
                            height: 1200,
                        })
                        .with_title("Demo"),
                )
                .unwrap(),
        );

        let gfx = pollster::block_on(Graphics::new(Arc::clone(&window)));
        let assets = Assets::load(&gfx);
        let mut scene = Scene::new();
        let mut physics = Physics::new();
        let input = Input::new();
        let frame_time = FrameTime::new();

        // Player is outside the normal components set for convenience because it's a singleton.
        // Ideally it should be unified with the rest of the objects once we have a proper ECS
        // or an alternative.
        let player = Player::new(&gfx, &mut physics);
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

        window.request_redraw();

        self.window = Some(window);
        self.assets = Some(assets);
        self.gfx = Some(gfx);
        self.physics = Some(physics);
        self.input = Some(input);
        self.frame_time = Some(frame_time);
        self.scene = Some(scene);
        self.player = Some(player);
        self.pp_cam = Some(pp_cam);
        self.pp_id = pp_idx;
        self.player_target_id = player_target_idx;
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        if self.window.is_none() || id != self.window.as_ref().unwrap().id() {
            return;
        }

        match event {
            WindowEvent::RedrawRequested => {
                // TODO Refactor this ugliness
                let mut scene = self.scene.take().unwrap();
                let mut physics = self.physics.take().unwrap();
                let mut player = self.player.take().unwrap();
                let mut gfx = self.gfx.take().unwrap();
                let mut input = self.input.take().unwrap();
                let window = self.window.take().unwrap();

                if input.action_activated(InputAction::Escape) {
                    event_loop.exit();
                }

                if let Some(&size) = self.new_canvas_size.as_ref() {
                    gfx.resize(size);
                }

                let dt = self.frame_time.as_mut().unwrap().advance();

                physics.update(dt);

                player.update(
                    &gfx,
                    dt,
                    &input,
                    &window,
                    &mut physics,
                    &self.new_canvas_size,
                );

                // Spawn a single box automatically
                {
                    if input.action_activated(InputAction::Spawn) || !self.spawned_demo_box {
                        let pos = if self.spawned_demo_box {
                            player.transform().position() + player.transform().forward().xyz() * 5.0
                        } else {
                            self.spawned_demo_box = true;
                            Vec3::y_axis().xyz() * 5.0
                        };
                        scene.spawn_cube(
                            pos,
                            Vec3::from_element(1.0),
                            &gfx,
                            self.assets.as_ref().unwrap(),
                            &mut physics,
                        );
                    }
                }

                scene.update_grabbed(&player, &input, &mut physics);
                scene.update_player_target(&player, self.player_target_id);

                scene.sync_physics(&physics);

                if self.new_canvas_size.is_some() {
                    scene.update_post_process_overlay(
                        self.pp_id,
                        player.camera().target().as_ref().unwrap().color_tex(),
                        &gfx,
                        self.assets.as_ref().unwrap(),
                    );
                }

                render_pass(
                    &gfx,
                    &scene.build_render_bundles(player.camera(), player.transform(), &gfx),
                    player.camera().target().as_ref(),
                );

                render_pass(
                    &gfx,
                    &scene.build_render_bundles(
                        self.pp_cam.as_ref().unwrap(),
                        &Transform::default(),
                        &gfx,
                    ),
                    None,
                );

                input.clear();
                // TODO Review - needed? Is there a better way?
                window.request_redraw();

                self.player = Some(player);
                self.physics = Some(physics);
                self.input = Some(input);
                self.window = Some(window);
                self.gfx = Some(gfx);
                self.scene = Some(scene);
                self.new_canvas_size = None;
            }

            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: Code(code),
                        state,
                        ..
                    },
                ..
            } => {
                self.input
                    .as_mut()
                    .unwrap()
                    .handle_keyboard_event(code, state == ElementState::Pressed);
            }

            WindowEvent::MouseInput { button, state, .. } => {
                self.input
                    .as_mut()
                    .unwrap()
                    .handle_mouse_button_event(button, state == ElementState::Pressed);
            }

            WindowEvent::CloseRequested => event_loop.exit(),

            WindowEvent::Resized(size) => self.new_canvas_size = Some(size),

            _ => {}
        }
    }

    fn device_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _device_id: DeviceId,
        event: DeviceEvent,
    ) {
        match event {
            DeviceEvent::MouseMotion { delta } => self
                .input
                .as_mut()
                .unwrap()
                .handle_mouse_move_event(delta.0 as f32, delta.1 as f32),

            DeviceEvent::MouseWheel { .. } => (),

            _ => (),
        };
    }
}

fn main() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut state = State::default();
    if let Err(e) = event_loop.run_app(&mut state) {
        eprintln!("Error: {e}");
    }
}
