use hecs::World;
use wgpu::RenderBundle;
use winit::dpi::PhysicalSize;
use winit::event::{DeviceEvent, ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::platform::run_return::EventLoopExtRunReturn;
use winit::window::{Window, WindowBuilder};

use new::DebugUI;
use new::Device;
use new::FrameTime;
use new::Input;
use new::PhysicsWorld;
use new::RenderTarget;
use new::SurfaceSize;

use crate::new::{
    ApplyMaterial, Assets, Camera, DrawMesh, FloorBox, Material, Mesh, PhysicsBody, Player,
    PlayerTarget, RenderTags, Skybox, Transform,
};

mod new;

fn new_bundle_encoder<'a>(
    device: &'a Device,
    target: Option<&RenderTarget>,
) -> wgpu::RenderBundleEncoder<'a> {
    let color_format = target.map_or(device.surface_texture_format(), |t| t.color_tex().format());
    let depth_format = target.map_or(device.depth_texture_format(), |t| t.depth_tex().format());

    device.create_render_bundle_encoder(&wgpu::RenderBundleEncoderDescriptor {
        label: None,
        multiview: None,
        sample_count: 1,
        color_formats: &[Some(color_format)],
        depth_stencil: Some(wgpu::RenderBundleDepthStencil {
            format: depth_format,
            depth_read_only: false,
            stencil_read_only: false,
        }),
    })
}

fn to_render_bundle<'a>(
    mesh: &'a Mesh,
    mat: &'a mut Material,
    tr: &'a Transform,
    camera: (&Camera, &Transform),
    device: &Device,
) -> RenderBundle {
    let mut encoder = new_bundle_encoder(device, camera.0.target().as_ref());
    mat.apply(&mut encoder, device, camera, tr);
    encoder.draw_mesh(mesh);
    encoder.finish(&wgpu::RenderBundleDescriptor { label: None })
}

fn render_pass(
    device: &Device,
    bundles: &[RenderBundle],
    target: Option<&RenderTarget>,
    debug_ui: &mut DebugUI,
) {
    let surface_tex = target.is_none().then(|| {
        device
            .surface()
            .get_current_texture()
            .expect("Missing surface texture")
    });
    let surface_tex_view = surface_tex.as_ref().map(|t| {
        t.texture
            .create_view(&wgpu::TextureViewDescriptor::default())
    });

    let color_tex_view = target
        .map(|t| t.color_tex().view())
        .or(surface_tex_view.as_ref())
        .unwrap();
    let color_attachment = Some(wgpu::RenderPassColorAttachment {
        view: color_tex_view,
        resolve_target: None,
        ops: wgpu::Operations {
            load: wgpu::LoadOp::Clear(wgpu::Color::RED),
            store: true,
        },
    });

    let depth_tex_view = target
        .map(|t| t.depth_tex().view())
        .unwrap_or(device.depth_tex().view());
    let depth_attachment = Some(wgpu::RenderPassDepthStencilAttachment {
        view: depth_tex_view,
        depth_ops: Some(wgpu::Operations {
            load: wgpu::LoadOp::Clear(1.0),
            store: true,
        }),
        stencil_ops: None,
    });

    let cmd_buffer = {
        let mut encoder =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[color_attachment],
                depth_stencil_attachment: depth_attachment,
            });

            pass.execute_bundles(bundles.iter());
            debug_ui.render(device, &mut pass)
        }

        encoder.finish()
    };

    device.queue().submit(Some(cmd_buffer));
    if let Some(t) = surface_tex {
        t.present()
    }
}

struct OsEvents {
    close_requested: bool,
    new_surface_size: Option<SurfaceSize>,
}

fn handle_os_events(
    event_loop: &mut EventLoop<()>,
    input: &mut Input,
    window: &Window,
    debug_ui: &mut DebugUI,
) -> OsEvents {
    let mut events = OsEvents {
        new_surface_size: None,
        close_requested: false,
    };

    event_loop.run_return(|event, _, flow| {
        *flow = ControlFlow::Poll;

        match event {
            Event::MainEventsCleared => {
                *flow = ControlFlow::Exit;
            }

            Event::DeviceEvent {
                event: DeviceEvent::MouseMotion { delta },
                ..
            } => input.on_mouse_move((delta.0 as f32, delta.1 as f32)),

            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => match event {
                WindowEvent::CloseRequested => events.close_requested = true,

                WindowEvent::MouseInput { state, button, .. } => {
                    input.on_mouse_button(*button, *state == ElementState::Pressed)
                }

                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            state,
                            virtual_keycode: Some(keycode),
                            ..
                        },
                    ..
                } => {
                    if *keycode == VirtualKeyCode::Escape && *state == ElementState::Pressed {
                        events.close_requested = true;
                    }
                    input.on_key(*keycode, *state == ElementState::Pressed);
                }

                WindowEvent::Resized(new_size) => {
                    events.new_surface_size = Some(*new_size);
                }

                WindowEvent::ScaleFactorChanged {
                    new_inner_size: new_size,
                    ..
                } => {
                    events.new_surface_size = Some(**new_size);
                }

                _ => (),
            },

            _ => {}
        }

        debug_ui.handle_window_event(window, &event);
    });

    events
}

fn main() {
    let mut event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Demo")
        .with_inner_size(PhysicalSize {
            width: 1900,
            height: 1200,
        })
        .build(&event_loop)
        .unwrap();
    let mut device = pollster::block_on(async { Device::new(&window).await });
    let mut physics = PhysicsWorld::new();
    let mut input = Input::new();
    let mut frame_time = FrameTime::new();
    let mut debug_ui = DebugUI::new(&device, &window);
    let assets = Assets::load(&device);

    let mut world = World::new();

    Skybox::spawn(&device, &assets, &mut world);
    FloorBox::spawn(&device, &mut physics, &assets, &mut world);
    PlayerTarget::spawn(&device, &assets, &mut world);
    Player::spawn(&device, &mut physics, &mut world);

    loop {
        frame_time.update();
        input.reset();

        let events = handle_os_events(&mut event_loop, &mut input, &window, &mut debug_ui);

        if events.close_requested {
            break;
        }

        // Resize
        if let Some(new_size) = events.new_surface_size {
            device.resize(new_size);
            // TODO Remove, this is temp
            Player::resize(new_size, &device, &mut world);
        }

        debug_ui.prepare_render(&window, frame_time.delta, |frame| {
            frame
                .window("Debug info")
                .position([10.0, 10.0], imgui::Condition::FirstUseEver)
                .movable(false)
                .resizable(false)
                .always_auto_resize(true)
                .collapsible(false)
                .no_decoration()
                .build(|| {
                    frame.text(
                        "Controls:\n\
                         - Toggle camera control: Tab\n\
                         - Move: WASDQE\n\
                         - Grab and release objects: LMB\n\
                         - Spawn new box: Space\n\
                         - Quit: Escape",
                    );

                    let mut mouse_pos = frame.io().mouse_pos;
                    // Prevent UI jumping at start when the mouse position is not yet known
                    // and imgui returns extra huge numbers.
                    if !(-10000.0f32..10000.0f32).contains(&mouse_pos[0]) {
                        mouse_pos = [-1.0f32, -1.0f32];
                    }
                    frame.text(format!(
                        "Mouse position: ({:.1},{:.1})",
                        mouse_pos[0], mouse_pos[1]
                    ));
                });
        });

        // TODO Run at fixed steps
        physics.update(frame_time.delta);

        Player::update(&frame_time, &input, &window, &mut physics, &mut world);

        // TODO Grabbing (can test it on the floor box, no need to add spawning first).
        // TODO Spawning boxes.
        // TODO Render order.

        // Update player target
        PlayerTarget::update(&mut world);
        PhysicsBody::sync_to_transforms(&physics, &mut world);

        // Render
        let mut bundles = Vec::<RenderBundle>::new();

        let mut q = world.query::<(&Player, &Transform, &Camera)>();
        let (_, (_, player_tr, player_cam)) = q.iter().next().unwrap();
        for (_, (mesh, mat, tr, tags)) in world
            .query::<(&Mesh, &mut Material, &Transform, &RenderTags)>()
            .iter()
        {
            if !player_cam.should_render(tags.0) {
                continue;
            }

            bundles.push(to_render_bundle(
                mesh,
                mat,
                tr,
                (&player_cam, &player_tr),
                &device,
            ));
        }

        // TODO Per camera (if needed)
        render_pass(&device, &bundles[..], None, &mut debug_ui);
    }
}
