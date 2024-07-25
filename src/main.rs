use std::sync::Arc;

use winit::application::ApplicationHandler;
use winit::dpi::PhysicalSize;
use winit::event::{DeviceEvent, DeviceId, ElementState, KeyEvent, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::PhysicalKey::Code;
use winit::window::{Window, WindowId};

use crate::assets::Assets;
use crate::frame_time::FrameTime;
use crate::graphics::{Graphics, SurfaceSize};
use crate::input::{Input, InputAction};
use crate::scene::Scene;

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
    input: Option<Input>,
    frame_time: Option<FrameTime>,
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

        window.request_redraw();

        self.scene = Some(Scene::new(&gfx, &assets));
        self.frame_time = Some(FrameTime::new());
        self.input = Some(Input::new());
        self.window = Some(window);
        self.assets = Some(assets);
        self.gfx = Some(gfx);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        if self.window.is_none() || id != self.window.as_ref().unwrap().id() {
            return;
        }

        match event {
            WindowEvent::RedrawRequested => {
                // TODO Avoid this moving-out-and-moving-back-in
                let mut scene = self.scene.take().unwrap();
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

                scene.update(
                    dt,
                    &gfx,
                    &input,
                    &window,
                    self.assets.as_ref().unwrap(),
                    &self.new_canvas_size,
                );

                scene.render(&gfx);

                input.clear();
                // TODO Needed? Is there a better way?
                window.request_redraw();

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
