use imgui::Condition;
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
use crate::ui::Ui;

mod assets;
mod components;
mod file;
mod frame_time;
mod graphics;
mod input;
mod materials;
mod math;
mod mesh;
mod physics;
mod render_target;
mod scene;
mod texture;
mod ui;
mod vertex;

// TODO Switch to raw Vulkan? It at least has stable API.
// TODO egui (https://github.com/ejb004/egui-wgpu-demo) or other UI.
// Currently they all seem unusable after the recent update of winit to versions 0.29 - 0.30

// TODO Spawned boxes should be rotated based on the camera view.
// TODO Dragging should maintain box rotation relative to the camera.
// TODO Selected object highlighting.
// TODO Gizmos (e.g. axes instead of a box representing the player's target)

#[derive(Default)]
struct State<'a> {
    window: Option<Arc<Window>>,
    gfx: Option<Graphics<'a>>,
    assets: Option<Assets>,
    scene: Option<Scene>,
    input: Option<Input>,
    frame_time: Option<FrameTime>,
    new_canvas_size: Option<SurfaceSize>,
    ui: Option<Ui>,
}

impl State<'_> {
    fn render(&mut self, event_loop: &ActiveEventLoop) {
        // TODO Avoid this moving-out-and-moving-back-in
        let mut scene = self.scene.take().unwrap();
        let mut gfx = self.gfx.take().unwrap();
        let mut input = self.input.take().unwrap();
        let mut assets = self.assets.take().unwrap();
        let mut ui = self.ui.take().unwrap();
        let window = self.window.take().unwrap();

        if input.action_activated(InputAction::Quit) {
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
            &mut assets,
            &self.new_canvas_size,
        );

        ui.prepare_frame(dt, &window, |frame| {
            let window = frame.window("Hello world");
            window
                .size([300.0, 100.0], Condition::FirstUseEver)
                .build(|| {
                    frame.text("Hello world!");
                    frame.text("This...is...imgui-rs on WGPU!");
                    frame.separator();
                    let mouse_pos = frame.io().mouse_pos;
                    frame.text(format!(
                        "Mouse Position: ({:.1},{:.1})",
                        mouse_pos[0], mouse_pos[1]
                    ));
                });

            let window = frame.window("Hello too");
            window
                .size([400.0, 200.0], Condition::FirstUseEver)
                .position([400.0, 200.0], Condition::FirstUseEver)
                .build(|| {
                    frame.text(format!("Frame time: {dt:?}"));
                });
        });

        scene.render(&gfx, &mut assets);
        gfx.render_ui(&mut ui);

        input.clear();
        // TODO Needed? Is there a better way?
        window.request_redraw();

        self.assets = Some(assets);
        self.input = Some(input);
        self.window = Some(window);
        self.gfx = Some(gfx);
        self.scene = Some(scene);
        self.ui = Some(ui);
        self.new_canvas_size = None;
    }
}

impl ApplicationHandler for State<'_> {
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
        let mut assets = Assets::load(&gfx);

        let ui = Ui::new(
            &gfx,
            gfx.queue(),
            window.as_ref(),
            window.scale_factor(),
            gfx.surface_texture_format(),
        );

        window.request_redraw();

        self.scene = Some(Scene::new(&gfx, &mut assets));
        self.frame_time = Some(FrameTime::new());
        self.input = Some(Input::new());
        self.window = Some(window);
        self.assets = Some(assets);
        self.gfx = Some(gfx);
        self.ui = Some(ui);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        if self.window.is_none() || id != self.window.as_ref().unwrap().id() {
            return;
        }

        match event {
            WindowEvent::RedrawRequested => self.render(event_loop),

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
                    .consume_keyboard_event(code, state == ElementState::Pressed);
            }

            WindowEvent::MouseInput { button, state, .. } => {
                self.input
                    .as_mut()
                    .unwrap()
                    .consume_mouse_button_event(button, state == ElementState::Pressed);
            }

            WindowEvent::CursorEntered { .. } => {
                self.input.as_mut().unwrap().consume_cursor_entrance(true);
            }

            WindowEvent::CursorLeft { .. } => {
                self.input.as_mut().unwrap().consume_cursor_entrance(false);
            }

            WindowEvent::CursorMoved { position, .. } => {
                self.input
                    .as_mut()
                    .unwrap()
                    .consume_cursor_position(position.x as f32, position.y as f32);
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
                .consume_mouse_delta(delta.0 as f32, delta.1 as f32),

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
