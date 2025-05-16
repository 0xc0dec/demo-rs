use crate::frame_time::FrameTime;
use crate::input::{Input, InputAction};
use crate::render::{Renderer, SurfaceSize};
use crate::scene::Assets;
use crate::scene::Scene;
use crate::state::State;
use futures_lite::future;
use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::dpi::PhysicalSize;
use winit::event::{DeviceEvent, DeviceId, Event, WindowEvent};
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowId};

#[derive(Default)]
pub struct App<'a> {
    state: Option<State<'a>>,
    assets: Option<Assets>,
    scene: Option<Scene>,
    frame_time: Option<FrameTime>,
    new_canvas_size: Option<SurfaceSize>,
}

impl App<'_> {
    fn update_and_render(&mut self, event_loop: &ActiveEventLoop) {
        // TODO Avoid this ugliness.
        let mut state = self.state.take().unwrap();
        let mut scene = self.scene.take().unwrap();
        let mut assets = self.assets.take().unwrap();

        if state.input.action_activated(InputAction::Quit) {
            event_loop.exit();
        }

        let dt = self.frame_time.as_mut().unwrap().advance();

        state.renderer.update(self.new_canvas_size);
        scene.update(dt, &state, &mut assets, &self.new_canvas_size);
        scene.render(&state.renderer, &mut assets);

        state.input.clear();
        state.window.request_redraw();

        self.state = Some(state);
        self.assets = Some(assets);
        self.scene = Some(scene);
        self.new_canvas_size = None;
    }
}

impl ApplicationHandler for App<'_> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // This function should be re-entrant, see the docs. Exiting if already initialized.
        if self.state.is_some() {
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
        window.request_redraw();

        let rr = future::block_on(Renderer::new(Arc::clone(&window)));
        let mut assets = Assets::load(&rr);

        let state = State {
            window,
            renderer: rr,
            input: Input::new(),
        };

        self.scene = Some(Scene::new(&state, &mut assets));
        self.frame_time = Some(FrameTime::new());
        self.assets = Some(assets);
        self.state = Some(state);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        if self.state.is_none() || id != self.state.as_ref().unwrap().window.id() {
            return;
        }

        match &event {
            WindowEvent::RedrawRequested => self.update_and_render(event_loop),
            WindowEvent::Resized(size) => self.new_canvas_size = Some(*size),
            WindowEvent::CloseRequested => event_loop.exit(),
            _ => {}
        }

        self.state
            .as_mut()
            .unwrap()
            .input
            .handle_window_event(&event);

        // TODO Avoid having to pass events to scene, this is currently needed just for UI.
        self.scene.as_mut().unwrap().handle_event(
            &Event::WindowEvent {
                window_id: id,
                event,
            },
            self.state.as_ref().unwrap(),
        )
    }

    fn device_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _device_id: DeviceId,
        event: DeviceEvent,
    ) {
        self.state
            .as_mut()
            .unwrap()
            .input
            .handle_device_event(&event);
    }
}
