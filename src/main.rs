use imgui::Condition;
use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::dpi::PhysicalSize;
use winit::event::{DeviceEvent, DeviceId, Event, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

use crate::assets::Assets;
use crate::frame_time::FrameTime;
use crate::input::{Input, InputAction};
use crate::renderer::{Renderer, SurfaceSize};
use crate::scene::Scene;
use crate::ui::Ui;

mod assets;
mod components;
mod file;
mod frame_time;
mod input;
mod materials;
mod math;
mod mesh;
mod physics;
mod render_target;
mod renderer;
mod scene;
mod texture;
mod ui;
mod vertex;

// TODO Switch to raw Vulkan? It at least has stable API.
// TODO Fix mouse first person rotation, it feels off.
// TODO Spawned boxes should be rotated based on the camera view.
// TODO Dragging should maintain box rotation relative to the camera.
// TODO Selected object highlighting.
// TODO Gizmos (e.g. axes instead of a box representing the player's target)

#[derive(Default)]
struct State<'a> {
    window: Option<Arc<Window>>,
    renderer: Option<Renderer<'a>>,
    assets: Option<Assets>,
    scene: Option<Scene>,
    input: Option<Input>,
    frame_time: Option<FrameTime>,
    new_canvas_size: Option<SurfaceSize>,
    ui: Option<Ui>,
}

impl State<'_> {
    fn render(&mut self, event_loop: &ActiveEventLoop) {
        // TODO Avoid this ugliness.
        let mut scene = self.scene.take().unwrap();
        let mut rr = self.renderer.take().unwrap();
        let mut input = self.input.take().unwrap();
        let mut assets = self.assets.take().unwrap();
        let mut ui = self.ui.take().unwrap();
        let window = self.window.take().unwrap();

        if input.action_activated(InputAction::Quit) {
            event_loop.exit();
        }

        if let Some(&size) = self.new_canvas_size.as_ref() {
            rr.resize(size);
        }

        let dt = self.frame_time.as_mut().unwrap().advance();

        scene.update(dt, &rr, &input, &window, &mut assets, &self.new_canvas_size);

        ui.prepare_frame(dt, &window, |frame| {
            let window = frame.window("Info");
            window
                .size([300.0, 150.0], Condition::FirstUseEver)
                .build(|| {
                    frame.text("Hello world!");
                    frame.text("This...is...imgui-rs on WGPU!");
                    frame.separator();
                    let mouse_pos = frame.io().mouse_pos;
                    frame.text(format!(
                        "Mouse position: ({:.1},{:.1})",
                        mouse_pos[0], mouse_pos[1]
                    ));
                    frame.text(format!("Frame time: {dt:?}"));
                });
        });

        scene.render(&rr, &mut assets, &mut ui);

        input.clear();
        window.request_redraw();

        self.assets = Some(assets);
        self.input = Some(input);
        self.window = Some(window);
        self.renderer = Some(rr);
        self.scene = Some(scene);
        self.ui = Some(ui);
        self.new_canvas_size = None;
    }
}

impl ApplicationHandler for State<'_> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // This function should be re-entrant, see the docs. Existing if already initialized.
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

        let rr = pollster::block_on(Renderer::new(Arc::clone(&window)));
        let mut assets = Assets::load(&rr);
        let ui = Ui::new(&rr, window.as_ref(), window.scale_factor());

        window.request_redraw();

        self.scene = Some(Scene::new(&rr, &mut assets));
        self.frame_time = Some(FrameTime::new());
        self.input = Some(Input::new());
        self.window = Some(window);
        self.assets = Some(assets);
        self.renderer = Some(rr);
        self.ui = Some(ui);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        if self.window.is_none() || id != self.window.as_ref().unwrap().id() {
            return;
        }

        match &event {
            WindowEvent::RedrawRequested => self.render(event_loop),
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => self.new_canvas_size = Some(*size),
            _ => {}
        }

        self.input.as_mut().unwrap().handle_window_event(&event);

        self.ui.as_mut().unwrap().handle_event::<()>(
            &Event::WindowEvent {
                window_id: id,
                event,
            },
            self.window.as_ref().unwrap(),
        );
    }

    fn device_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _device_id: DeviceId,
        event: DeviceEvent,
    ) {
        self.input.as_mut().unwrap().handle_device_event(&event);
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
