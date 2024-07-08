use crate::components::{
    Material, Mesh, PhysicsBody, PhysicsBodyParams, Player, RenderOrder, RenderTags, Transform,
    RENDER_TAG_SCENE,
};
use crate::debug_ui::DebugUI;
use crate::events::{KeyboardEvent, MouseEvent, ResizeEvent};
use crate::math::Vec3;
use std::sync::Arc;
use winit::dpi::PhysicalSize;
use winit::event::{DeviceEvent, ElementState, Event, KeyboardInput, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::platform::run_return::EventLoopExtRunReturn;
use winit::window::{Window, WindowBuilder};

use crate::resources::*;
use crate::systems::{mesh_to_render_bundle, render_pass};

mod assets;
mod components;
mod debug_ui;
mod events;
mod math;
mod resources;
mod systems;

// TODO After removing Bevy, refactor file structure, remove the notion of components/resources/systems.

fn consume_system_events(
    event_loop: &mut EventLoop<()>,
    window: &Window,
    debug_ui: &mut DebugUI,
    mouse_events: &mut Vec<MouseEvent>,
    keyboard_events: &mut Vec<KeyboardEvent>,
    resize_events: &mut Vec<ResizeEvent>,
) {
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
                mouse_events.push(MouseEvent::Move {
                    dx: delta.0 as f32,
                    dy: delta.1 as f32,
                });
            }

            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => match event {
                WindowEvent::MouseInput { state, button, .. } => {
                    mouse_events.push(MouseEvent::Button {
                        btn: *button,
                        pressed: *state == ElementState::Pressed,
                    });
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
                    keyboard_events.push(KeyboardEvent {
                        code: *keycode,
                        pressed: *key_state == ElementState::Pressed,
                    });
                }

                WindowEvent::Resized(new_size) => {
                    resize_events.push(ResizeEvent(*new_size));
                }

                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    resize_events.push(ResizeEvent(**new_inner_size));
                }

                _ => (),
            },

            _ => {}
        }

        // TODO Make DebugUI consume events from the event vectors we're filling in this function.
        debug_ui.handle_window_event(window, &event);
    });
}

fn build_debug_ui(ui: &mut DebugUI, frame_time: &FrameTime, window: &Window) {
    ui.prepare_render(window, frame_time.delta, |frame| {
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
                    - Grab objects: hold LMB\n\
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
}

struct Components {
    transforms: Vec<Option<Transform>>,
    meshes: Vec<Option<Mesh>>,
    materials: Vec<Option<Material>>,
    bodies: Vec<Option<PhysicsBody>>,
    render_orders: Vec<Option<RenderOrder>>,
    render_tags: Vec<Option<RenderTags>>,
}

impl Components {
    fn new() -> Self {
        Self {
            transforms: Vec::new(),
            meshes: Vec::new(),
            materials: Vec::new(),
            bodies: Vec::new(),
            render_orders: Vec::new(),
            render_tags: Vec::new(),
        }
    }

    fn spawn_mesh(
        &mut self,
        transform: Transform,
        mesh: Mesh,
        material: Material,
        body: Option<PhysicsBody>,
        render_order: Option<RenderOrder>,
        render_tags: Option<RenderTags>,
    ) -> usize {
        self.transforms.push(Some(transform));
        self.meshes.push(Some(mesh));
        self.materials.push(Some(material));
        self.bodies.push(body);
        self.render_orders.push(render_order);
        self.render_tags.push(render_tags);
        self.transforms.len() - 1
    }
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
    let mut device = pollster::block_on(Device::new(&window));
    let mut physics = PhysicsWorld::new();
    let mut input = Input::new();
    let mut frame_time = FrameTime::new();

    let assets = Assets::load(&device);
    let mut debug_ui = DebugUI::new(&device, &window);

    // TODO More optimal, avoid vec cleanup on each iteration
    let mut mouse_events = Vec::new();
    let mut keyboard_events = Vec::new();
    let mut resize_events = Vec::new();

    // TODO Replace with a proper ECS or restructure in some other better way
    let mut components = Components::new();

    // Player is outside the normal components set for convenience because it's a singleton.
    // Ideally it should be unified with the rest of the objects once we have a proper ECS
    // or an alternative.
    let mut player = Player::new(&device, &mut physics);

    let _skybox_id = components.spawn_mesh(
        Transform::default(),
        Mesh(Arc::new(assets::Mesh::quad(&device))),
        Material::skybox(&device, &assets, &assets.skybox_tex),
        None,
        Some(RenderOrder(-100)),
        Some(RenderTags(RENDER_TAG_SCENE)),
    );

    let _floor_id = {
        let pos = Vec3::from_element(0.0);
        let scale = Vec3::new(10.0, 0.5, 10.0);
        components.spawn_mesh(
            Transform::new(pos, scale),
            Mesh(Arc::clone(&assets.box_mesh)),
            Material::diffuse(&device, &assets, &assets.stone_tex),
            Some(PhysicsBody::new(
                PhysicsBodyParams {
                    pos,
                    scale,
                    rotation_axis: Vec3::from_element(0.0),
                    rotation_angle: 0.0,
                    movable: false,
                },
                &mut physics,
            )),
            None,
            Some(RenderTags(RENDER_TAG_SCENE)),
        )
    };

    while !input.action_active(InputAction::Escape) {
        consume_system_events(
            &mut event_loop,
            &window,
            &mut debug_ui,
            &mut mouse_events,
            &mut keyboard_events,
            &mut resize_events,
        );

        let last_resize_event = resize_events.last();

        if let Some(e) = last_resize_event {
            device.resize(e.0);
        }

        input.update(&mouse_events, &keyboard_events);
        frame_time.update();
        physics.update(frame_time.delta);

        player.update(
            &device,
            &frame_time,
            &input,
            &window,
            &mut physics,
            last_resize_event,
        );

        build_debug_ui(&mut debug_ui, &frame_time, &window);

        // TODO Sort by render order
        // TODO Group meshes into bundles?
        let bundles = (0..components.meshes.len())
            .map(|idx| {
                mesh_to_render_bundle(
                    components.meshes.get(idx).unwrap().as_ref().unwrap(),
                    components.materials.get_mut(idx).unwrap().as_mut().unwrap(),
                    components.transforms.get(idx).unwrap().as_ref().unwrap(),
                    (&player.camera, &player.transform),
                    &device,
                )
            })
            .collect::<Vec<_>>();

        render_pass(&device, &bundles, None, Some(&mut debug_ui));

        mouse_events.clear();
        keyboard_events.clear();
        resize_events.clear();
    }
}
