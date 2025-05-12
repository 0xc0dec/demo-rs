use crate::assets::Assets;
use crate::frame_time::FrameTime;
use crate::input::{Input, InputAction};
use crate::renderer::{Renderer, SurfaceSize};
use crate::scene::Scene;
use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::dpi::PhysicalSize;
use winit::event::{DeviceEvent, DeviceId, Event, WindowEvent};
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowId};

#[derive(Default)]
pub struct App<'a> {
    window: Option<Arc<Window>>,
    renderer: Option<Renderer<'a>>,
    assets: Option<Assets>,
    scene: Option<Scene>,
    input: Option<Input>,
    frame_time: Option<FrameTime>,
    new_canvas_size: Option<SurfaceSize>,
}

impl App<'_> {
    fn render(&mut self, event_loop: &ActiveEventLoop) {
        // TODO Avoid this ugliness.
        let mut scene = self.scene.take().unwrap();
        let mut rr = self.renderer.take().unwrap();
        let mut input = self.input.take().unwrap();
        let mut assets = self.assets.take().unwrap();
        let window = self.window.take().unwrap();

        if input.action_activated(InputAction::Quit) {
            event_loop.exit();
        }

        if let Some(&size) = self.new_canvas_size.as_ref() {
            rr.resize(size);
        }

        let dt = self.frame_time.as_mut().unwrap().advance();

        scene.update(dt, &rr, &input, &window, &mut assets, &self.new_canvas_size);
        scene.render(&rr, &mut assets);

        input.clear();
        window.request_redraw();

        self.assets = Some(assets);
        self.input = Some(input);
        self.window = Some(window);
        self.renderer = Some(rr);
        self.scene = Some(scene);
        self.new_canvas_size = None;
    }
}

impl ApplicationHandler for App<'_> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // This function should be re-entrant, see the docs. Exiting if already initialized.
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

        window.request_redraw();

        self.scene = Some(Scene::new(&rr, &mut assets, &window));
        self.frame_time = Some(FrameTime::new());
        self.input = Some(Input::new());
        self.window = Some(window);
        self.assets = Some(assets);
        self.renderer = Some(rr);
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
        // TODO Avoid having to pass events to scene, this is currently needed just for UI.
        self.scene.as_mut().unwrap().handle_event(
            &Event::WindowEvent {
                window_id: id,
                event,
            },
            self.window.as_ref().unwrap(),
        )
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
