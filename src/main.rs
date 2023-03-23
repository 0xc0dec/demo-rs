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
mod assets;
mod scene;
mod shaders;
mod texture;
mod transform;
mod app;
mod frame_time;
mod scene2;
mod state;

use bevy_ecs::prelude::{IntoSystemConfig, NonSend, Schedule, World, run_once, Condition};
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
use input::Input;
use crate::assets::Assets;
use crate::frame_time::FrameTime;
use crate::physics_world::PhysicsWorld;
use crate::scene2::Scene2;
use crate::state::State;

fn before_update(
    window: NonSend<Window>,
    mut state: ResMut<State>,
    mut event_loop: NonSendMut<EventLoop<()>>,
    mut input: NonSendMut<Input>,
    mut device: NonSendMut<Device>,
    mut debug_ui: NonSendMut<DebugUI>
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

        debug_ui.handle_window_event(&window, &event);
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

    state.frame_time.update();
}

fn init(world: &mut World) {
    pollster::block_on(async {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_title("Demo")
            .with_inner_size(SurfaceSize::new(1800, 1200))
            .build(&event_loop)
            .unwrap();
        let device = Device::new(&window).await;
        let assets = Assets::new();
        let input = Input::new();
        let physics = PhysicsWorld::new();
        let debug_ui = DebugUI::new(&device, &window);

        world.insert_resource(State { running: true, frame_time: FrameTime::new() });
        world.insert_non_send_resource(physics);
        world.insert_non_send_resource(window);
        world.insert_non_send_resource(event_loop);
        world.insert_non_send_resource(device);
        world.insert_non_send_resource(assets);
        world.insert_non_send_resource(input);
        world.insert_non_send_resource(debug_ui);
    });
}

fn main() {
    let mut world = World::default();
    let mut schedule = Schedule::default();
    schedule
        .add_system(init.run_if(run_once()))
        .add_system(Scene2::init
            .after(init)
            .run_if(run_once())
        )
        .add_system(before_update.after(Scene2::init));
    // Scene2::configure_update_systems(&mut schedule);

    // let mut update = Schedule::default();
    // // TODO Ensure before_update always runs before all other updates
    // update.add_system(before_update);
    // Scene2::configure_update_systems(&mut update);

    loop {
        schedule.run(&mut world);

        if !world.get_resource::<State>().unwrap().running { return; }

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
