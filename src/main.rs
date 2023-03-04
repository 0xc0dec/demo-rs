mod scene;
mod texture;
mod camera;
mod transform;
mod input;
mod model;
mod resources;
mod driver;
mod material;
mod render_target;

use winit::{event::*, event_loop::{ControlFlow, EventLoop}, window::WindowBuilder};

use scene::Scene;
use input::Input;
use driver::Driver;
use render_target::RenderTarget;

async fn run() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Demo")
        .build(&event_loop)
        .unwrap();

    let mut driver = Driver::new(&window).await;
    let mut render_target = RenderTarget::new(&driver, wgpu::Color {
        r: 0.0,
        g: 0.5,
        b: 0.0,
        a: 1.0,
    });
    let mut scene = Scene::new(&driver).await;
    let mut input = Input::new();
    let mut time = instant::Instant::now();

    event_loop.run(move |event, _, flow| {
        *flow = ControlFlow::Poll;

        input.process_event(&event, &window.id());

        match event {
            Event::NewEvents(_) => {
                input.clear();
            }

            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => {
                match event {
                    WindowEvent::Resized(new_size) => {
                        driver.resize(Some(*new_size));
                        render_target.resize(&driver);
                    },
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        driver.resize(Some(**new_inner_size));
                        render_target.resize(&driver);
                    }
                    _ => {}
                }
            }

            Event::MainEventsCleared => {
                let dt = instant::Instant::now() - time;
                time = instant::Instant::now();

                scene.update(&input, dt.as_secs_f32());

                driver.render_frame(&mut scene, &render_target);

                // TODO Restore
                // match state.render(&renderer) {
                //     Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                //         renderer.resize(None);
                //         state.resize(&renderer);
                //     },
                //     Err(wgpu::SurfaceError::OutOfMemory) => *flow = ControlFlow::Exit,
                //     _ => {}
                // }
            }

            _ => {}
        }

        if input.escape_down {
            *flow = ControlFlow::Exit;
        }
    });
}

fn main() {
    pollster::block_on(run());
}
