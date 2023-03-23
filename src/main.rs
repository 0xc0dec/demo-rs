mod camera;
mod debug_ui;
mod device;
mod frame_context;
mod input;
mod math;
mod model;
mod physics_world;
mod post_processor;
mod render_target;
mod resources;
mod scene;
mod shaders;
mod texture;
mod transform;
mod app;

use std::collections::VecDeque;
use std::time::Instant;
use bevy_ecs::prelude::{NonSend, Res, Resource, Schedule, World};
use bevy_ecs::system::{NonSendMut, ResMut};
use winit::platform::run_return::EventLoopExtRunReturn;
use winit::window::{CursorGrabMode, Window};
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use debug_ui::DebugUI;
use device::Device;
use device::SurfaceSize;
use frame_context::FrameContext;
use input::Input;
use post_processor::PostProcessor;
use scene::Scene;
use crate::resources::Resources;
use crate::app::App;

#[derive(Resource)]
struct DeltaTime {
    dt: f32,
    queue: VecDeque<f32>,
    last_frame_instant: Instant
}

impl DeltaTime {
    const DT_FILTER_WIDTH: usize = 10;

    fn new() -> Self {
        let queue = VecDeque::with_capacity(Self::DT_FILTER_WIDTH);
        let last_frame_instant = Instant::now();

        Self {
            queue,
            last_frame_instant,
            dt: 0.0
        }
    }

    fn update(&mut self) {
        // Stolen from Kajiya
        let now = Instant::now();
        let dt_duration = now - self.last_frame_instant;
        self.last_frame_instant = now;

        let raw = dt_duration.as_secs_f32();

        if self.queue.len() >= DeltaTime::DT_FILTER_WIDTH {
            self.queue.pop_front();
        }
        self.queue.push_back(raw);

        self.dt = self.queue.iter().copied().sum::<f32>() / self.queue.len() as f32;
    }
}

#[derive(Resource)]
struct State {
    running: bool,
    dt: DeltaTime,
}

fn handle_events(
    window: NonSend<Window>,
    mut state: ResMut<State>,
    mut event_loop: NonSendMut<EventLoop<()>>,
    mut input: NonSendMut<Input>,
    mut device: NonSendMut<Device>
) {
    input.reset();

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
                input.on_mouse_move((delta.0 as f32, delta.1 as f32));
            }

            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => match event {
                WindowEvent::MouseInput { state, button, .. } => {
                    input.on_mouse_button(button, state);
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
                    input.on_key(keycode, key_state);
                }

                WindowEvent::Resized(new_size) => {
                    device.resize(*new_size);
                }

                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    device.resize(**new_inner_size);
                }

                _ => (),
            },

            _ => {}
        }

        // debug_ui.handle_window_event(&app.window, &event);
    });

    if input.escape_down {
        state.running = false;
    }

    // Grab/release cursor
    if input.rmb_down_just_switched {
        if input.rmb_down {
            window
                .set_cursor_grab(CursorGrabMode::Confined)
                .or_else(|_e| window.set_cursor_grab(CursorGrabMode::Locked))
                .unwrap();
            window.set_cursor_visible(false);
        } else {
            window.set_cursor_grab(CursorGrabMode::None).unwrap();
            window.set_cursor_visible(true);
        }
    }
}

fn update_dt(mut state: ResMut<State>) {
    state.dt.update();
}

async fn run() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Demo")
        .with_inner_size(SurfaceSize::new(1800, 1200))
        .build(&event_loop)
        .unwrap();
    let device = Device::new(&window).await;
    let resources = Resources::new();
    let input = Input::new();

    // let mut scene = Scene::new(&mut app).await;
    // let mut pp = PostProcessor::new(&device, None).await;

    // let mut debug_ui = DebugUI::new(&app);

    let mut world = World::default();

    let mut handle_events_schedule = Schedule::default();
    handle_events_schedule.add_system(handle_events);

    let mut update_schedule = Schedule::default();
    update_schedule.add_system(update_dt);

    world.insert_resource(State { running: true, dt: DeltaTime::new() });
    world.insert_non_send_resource(window);
    world.insert_non_send_resource(event_loop);
    world.insert_non_send_resource(device);
    world.insert_non_send_resource(input);

    while world.get_resource::<State>().unwrap().running {
        handle_events_schedule.run(&mut world);
        update_schedule.run(&mut world);

        // scene.update(&frame_context);
        // debug_ui.update(&frame_context);
        //
        // {
        //     let mut frame = app.device.new_frame(Some(pp.source_rt()));
        //     scene.render(&mut frame, &frame_context);
        //     frame.finish(None);
        // }
        //
        // {
        //     let mut frame = app.device.new_frame(None);
        //     pp.render(&mut frame);
        //     debug_ui.build_frame(&app.window, |frame| scene.build_debug_ui(frame));
        //     frame.finish(Some(&mut debug_ui));
        // }
    }
}

fn main() {
    pollster::block_on(run());
}
