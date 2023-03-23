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
struct DeltaTime(f32);

#[derive(Resource)]
struct State {
    running: bool
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

    // let mut app = App {
    //     window,
    //     resources,
    //     input
    // };

    // let mut scene = Scene::new(&mut app).await;
    // let mut pp = PostProcessor::new(&device, None).await;

    // let mut debug_ui = DebugUI::new(&app);

    let mut world = World::default();

    let mut handle_events_schedule = Schedule::default();
    handle_events_schedule.add_system(handle_events);

    let mut update_schedule = Schedule::default();

    world.insert_resource(State { running: true });
    world.insert_non_send_resource(window);
    world.insert_non_send_resource(event_loop);
    world.insert_non_send_resource(device);
    world.insert_non_send_resource(input);

    // schedule.add_system(update_scene);

    const DT_FILTER_WIDTH: usize = 10;
    let mut dt_queue: VecDeque<f32> = VecDeque::with_capacity(DT_FILTER_WIDTH);
    let mut last_frame_instant = std::time::Instant::now();

    loop {
        handle_events_schedule.run(&mut world);

        if !world.get_resource::<State>().unwrap().running {
            break;
        }

        // Stolen from Kajiya
        let dt = {
            let now = std::time::Instant::now();
            let dt_duration = now - last_frame_instant;
            last_frame_instant = now;

            let dt_raw = dt_duration.as_secs_f32();

            if dt_queue.len() >= DT_FILTER_WIDTH {
                dt_queue.pop_front();
            }

            dt_queue.push_back(dt_raw);
            dt_queue.iter().copied().sum::<f32>() / dt_queue.len() as f32
        };

        // let frame_context = FrameContext {
        //     dt,
        //     app: &app,
        // };

        // world.insert_resource(DeltaTime(dt));
        // schedule.run(&mut world);

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
