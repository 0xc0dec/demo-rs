use crate::assets::Texture;
use crate::components::{
    Camera, Material, Mesh, PhysicsBody, PhysicsBodyParams, Player, RenderTags, Transform,
    RENDER_TAG_DEBUG_UI, RENDER_TAG_HIDDEN, RENDER_TAG_POST_PROCESS, RENDER_TAG_SCENE,
};
use crate::debug_ui::DebugUI;
use crate::device::Device;
use crate::events::{KeyboardEvent, MouseEvent, ResizeEvent};
use crate::input::{Input, InputAction};
use crate::math::{to_point, Vec3};
use frame_time::FrameTime;
use physics_world::PhysicsWorld;
use render::build_render_bundle;
use render::render_pass;
use std::sync::Arc;
use wgpu::RenderBundle;
use winit::dpi::PhysicalSize;
use winit::event::{DeviceEvent, ElementState, Event, KeyboardInput, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::platform::run_return::EventLoopExtRunReturn;
use winit::window::{Window, WindowBuilder};

use crate::resources::*;

mod assets;
mod components;
mod debug_ui;
mod device;
mod events;
mod frame_time;
mod input;
mod math;
mod physics_world;
mod render;
mod resources;

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

// TODO A proper ECS or some other solution. This is a very basic solution for now.
struct Scene {
    transforms: Vec<Option<Transform>>,
    meshes: Vec<Option<Mesh>>,
    materials: Vec<Option<Material>>,
    bodies: Vec<Option<PhysicsBody>>,
    render_orders: Vec<i32>,
    render_tags: Vec<Option<RenderTags>>,
    grabbed_body_idx: Option<usize>,
    grabbed_body_player_local_pos: Option<Vec3>,
}

impl Scene {
    fn new() -> Self {
        Self {
            transforms: Vec::new(),
            meshes: Vec::new(),
            materials: Vec::new(),
            bodies: Vec::new(),
            render_orders: Vec::new(),
            render_tags: Vec::new(),
            grabbed_body_idx: None,
            grabbed_body_player_local_pos: None,
        }
    }

    fn spawn_mesh(
        &mut self,
        transform: Transform,
        mesh: Mesh,
        material: Material,
        body: Option<PhysicsBody>,
        render_order: Option<i32>,
        render_tags: Option<RenderTags>,
    ) -> usize {
        self.transforms.push(Some(transform));
        self.meshes.push(Some(mesh));
        self.materials.push(Some(material));
        self.bodies.push(body);
        self.render_orders.push(render_order.unwrap_or(0));
        self.render_tags.push(render_tags);
        self.transforms.len() - 1
    }

    fn spawn_floor(&mut self, device: &Device, assets: &Assets, physics: &mut PhysicsWorld) {
        let pos = Vec3::from_element(0.0);
        let scale = Vec3::new(10.0, 0.5, 10.0);
        self.spawn_mesh(
            Transform::new(pos, scale),
            Mesh(Arc::clone(&assets.box_mesh)),
            Material::diffuse(device, assets, &assets.stone_tex),
            Some(PhysicsBody::new(
                PhysicsBodyParams {
                    pos,
                    scale,
                    rotation_axis: Vec3::from_element(0.0),
                    rotation_angle: 0.0,
                    movable: false,
                },
                physics,
            )),
            None,
            Some(RenderTags(RENDER_TAG_SCENE)),
        );
    }

    fn spawn_cube(
        &mut self,
        pos: Vec3,
        scale: Vec3,
        device: &Device,
        assets: &Assets,
        physics: &mut PhysicsWorld,
    ) {
        self.spawn_mesh(
            Transform::new(pos, scale),
            Mesh(Arc::clone(&assets.box_mesh)),
            Material::diffuse(device, assets, &assets.stone_tex),
            Some(PhysicsBody::new(
                PhysicsBodyParams {
                    pos,
                    scale,
                    rotation_axis: Vec3::identity(),
                    rotation_angle: 0.0,
                    movable: true,
                },
                physics,
            )),
            None,
            Some(RenderTags(RENDER_TAG_SCENE)),
        );
    }

    fn spawn_skybox(&mut self, device: &Device, assets: &Assets) {
        self.spawn_mesh(
            Transform::default(),
            Mesh(Arc::new(assets::Mesh::quad(device))),
            Material::skybox(device, assets, &assets.skybox_tex),
            None,
            Some(-100),
            Some(RenderTags(RENDER_TAG_SCENE)),
        );
    }

    fn spawn_player_target(&mut self, device: &Device, assets: &Assets) -> usize {
        self.spawn_mesh(
            Transform::default(),
            Mesh(Arc::clone(&assets.box_mesh)),
            Material::color(device, assets),
            None,
            None,
            Some(RenderTags(RENDER_TAG_HIDDEN)),
        )
    }

    fn spawn_post_process_overlay(
        &mut self,
        source_color_tex: &Texture,
        device: &Device,
        assets: &Assets,
    ) -> usize {
        self.spawn_mesh(
            Transform::default(),
            Mesh(Arc::new(assets::Mesh::quad(device))),
            Material::post_process(device, assets, source_color_tex),
            None,
            Some(100),
            Some(RenderTags(RENDER_TAG_POST_PROCESS)),
        )
    }

    fn update_grabbed(&mut self, player: &Player, input: &Input, physics: &mut PhysicsWorld) {
        if input.action_active(InputAction::Grab) && player.controlled() {
            if self.grabbed_body_player_local_pos.is_none() {
                // Initiate grab
                if let Some(focus_body_handle) = player.focus_body() {
                    let body_idx = self
                        .bodies
                        .iter()
                        .position(|b| {
                            b.as_ref()
                                .is_some_and(|b| b.body_handle() == focus_body_handle)
                        })
                        .unwrap();
                    let body = self.bodies.get_mut(body_idx).unwrap().as_mut().unwrap();
                    body.set_kinematic(physics, true);
                    let body = physics.bodies.get_mut(focus_body_handle).unwrap();
                    let local_pos = player
                        .transform
                        .matrix()
                        .try_inverse()
                        .unwrap()
                        .transform_point(&to_point(*body.translation()))
                        .coords;
                    self.grabbed_body_idx = Some(body_idx);
                    self.grabbed_body_player_local_pos = Some(local_pos);
                }
            } else {
                // Update the grabbed object
                if let Some(grabbed_idx) = self.grabbed_body_idx {
                    let body = self.bodies.get_mut(grabbed_idx).unwrap().as_mut().unwrap();
                    let body = physics.bodies.get_mut(body.body_handle()).unwrap();
                    let new_pos = player
                        .transform
                        .matrix()
                        .transform_point(&to_point(self.grabbed_body_player_local_pos.unwrap()));
                    body.set_translation(new_pos.coords, true);
                }
            }
        } else {
            // Release grab
            if let Some(grabbed_idx) = self.grabbed_body_idx.take() {
                let body = self.bodies.get_mut(grabbed_idx).unwrap().as_mut().unwrap();
                body.set_kinematic(physics, false);
                self.grabbed_body_idx = None;
                self.grabbed_body_player_local_pos = None;
            }
        }
    }

    fn update_post_process_overlay(
        &mut self,
        idx: usize,
        source_color_tex: &Texture,
        device: &Device,
        assets: &Assets,
    ) {
        self.materials[idx] = Some(Material::post_process(device, assets, source_color_tex));
    }

    fn update_player_target(&mut self, player: &Player, target_idx: usize) {
        if let Some(player_focus_pt) = player.focus_point() {
            let dist_to_camera = (player.transform.position() - player_focus_pt).magnitude();
            let scale = (dist_to_camera / 10.0).min(0.1).max(0.01);

            let target_transform = self
                .transforms
                .get_mut(target_idx)
                .unwrap()
                .as_mut()
                .unwrap();
            target_transform.set_position(player_focus_pt);
            target_transform.set_scale(Vec3::from_element(scale));

            self.render_tags
                .get_mut(target_idx)
                .unwrap()
                .as_mut()
                .unwrap()
                .0 = RENDER_TAG_SCENE;
        } else {
            self.render_tags
                .get_mut(target_idx)
                .unwrap()
                .as_mut()
                .unwrap()
                .0 = RENDER_TAG_HIDDEN;
        }
    }

    // TODO Move elsewhere, it should not be a method on Scene
    fn build_render_bundles(
        &mut self,
        camera: &Camera,
        camera_transform: &Transform,
        device: &Device,
    ) -> Vec<RenderBundle> {
        let mut sorted_indices: Vec<(usize, i32)> = (0..self.meshes.len())
            .map(|idx| (idx, *self.render_orders.get(idx).unwrap()))
            .collect();
        sorted_indices
            .sort_by(|(_, ref order1), (_, ref order2)| order1.partial_cmp(order2).unwrap());
        let sorted_indices: Vec<usize> = sorted_indices.into_iter().map(|(idx, _)| idx).collect();

        // TODO Group meshes into bundles?
        sorted_indices
            .iter()
            .filter(|&&idx| {
                camera.should_render(
                    self.render_tags
                        .get(idx)
                        .unwrap()
                        .as_ref()
                        .map_or(0u32, |t| t.0),
                )
            })
            .map(|&idx| {
                build_render_bundle(
                    self.meshes.get(idx).unwrap().as_ref().unwrap(),
                    self.materials.get_mut(idx).unwrap().as_mut().unwrap(),
                    self.transforms.get(idx).unwrap().as_ref().unwrap(),
                    (&camera, &camera_transform),
                    device,
                )
            })
            .collect::<Vec<_>>()
    }

    fn sync_physics(&mut self, physics: &PhysicsWorld) {
        for idx in 0..self.bodies.len() {
            if let Some(body) = self.bodies.get(idx).unwrap() {
                let transform = self.transforms.get_mut(idx).unwrap().as_mut().unwrap();
                let body = physics.bodies.get(body.body_handle()).unwrap();
                let phys_pos = body.translation();
                let phys_rot = body.rotation().inverse(); // Not sure why inverse is needed
                transform.set(*phys_pos, *phys_rot.quaternion());
            }
        }
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
    // Store device + window in a new struct Device (or smth like that), add Deref traits to it.
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
    let mut scene = Scene::new();

    // Player is outside the normal components set for convenience because it's a singleton.
    // Ideally it should be unified with the rest of the objects once we have a proper ECS
    // or an alternative.
    let mut player = Player::new(&device, &mut physics);
    let player_target_idx = scene.spawn_player_target(&device, &assets);

    let pp_cam = Camera::new(1.0, RENDER_TAG_POST_PROCESS | RENDER_TAG_DEBUG_UI, None);

    scene.spawn_floor(&device, &assets, &mut physics);
    // Spawning skybox last to ensure the sorting by render order works and it still shows up
    // in the background.
    scene.spawn_skybox(&device, &assets);
    let pp_idx = scene.spawn_post_process_overlay(
        player.camera.target().as_ref().unwrap().color_tex(),
        &device,
        &assets,
    );

    let mut spawned_demo_box = false;

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

        if last_resize_event.is_some() {
            scene.update_post_process_overlay(
                pp_idx,
                player.camera.target().as_ref().unwrap().color_tex(),
                &device,
                &assets,
            );
        }

        scene.update_grabbed(&player, &input, &mut physics);
        scene.update_player_target(&player, player_target_idx);

        scene.sync_physics(&physics);

        if input.action_activated(InputAction::Spawn) || !spawned_demo_box {
            let pos = if spawned_demo_box {
                player.transform.position() + player.transform.forward().xyz() * 5.0
            } else {
                spawned_demo_box = true;
                Vec3::y_axis().xyz() * 5.0
            };

            scene.spawn_cube(pos, Vec3::from_element(1.0), &device, &assets, &mut physics);
        }

        // Render main scene into a texture
        render_pass(
            &device,
            &scene.build_render_bundles(&player.camera, &player.transform, &device),
            player.camera.target().as_ref(),
            None,
        );

        // Render post-process overlay + debug UI
        build_debug_ui(&mut debug_ui, &frame_time, &window);
        render_pass(
            &device,
            &scene.build_render_bundles(&pp_cam, &Transform::default(), &device),
            None,
            Some(&mut debug_ui),
        );

        mouse_events.clear();
        keyboard_events.clear();
        resize_events.clear();
    }
}
