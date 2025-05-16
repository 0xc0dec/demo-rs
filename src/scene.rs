use hecs::{Entity, World};
use imgui::Condition;
use winit::event::Event;

use crate::assets::Assets;
use crate::components::{
    Camera, Grab, Material, Mesh, Player, PlayerTarget, RenderOrder,
    RenderTags, RigidBody, RigidBodyParams, Transform, RENDER_TAG_DEBUG_UI, RENDER_TAG_POST_PROCESS,
    RENDER_TAG_SCENE,
};
use crate::input::InputAction;
use crate::materials;
use crate::math::Vec3;
use crate::physics::Physics;
use crate::renderer::{Renderer, SurfaceSize};
use crate::state::State;
use crate::ui::Ui;

pub struct Scene {
    world: World,
    physics: Physics,
    postprocessor: Entity,
    player: Entity,
    ui: Ui,
    spawned_startup_box: bool,
}

impl Scene {
    pub fn new(state: &State, assets: &mut Assets) -> Self {
        let mut world = World::new();
        let mut physics = Physics::new();

        // Player
        let player = Player::spawn(
            &mut world,
            &state.renderer,
            &mut physics,
            Vec3::new(7.0, 7.0, 7.0),
        );

        // Player target
        PlayerTarget::spawn(&state.renderer, &mut world, assets);

        // Skybox
        // Spawning skybox somewhere in the middle to ensure the sorting by render order works and it still shows up
        // in the background.
        let material = assets.add_skybox_material(&state.renderer, assets.skybox_texture);
        world.spawn((
            Transform::default(),
            Mesh(assets.quad_mesh),
            Material(material),
            RenderOrder(-100),
            RenderTags(RENDER_TAG_SCENE),
        ));

        // Post-processor
        let pp_src_tex = world
            .query_one_mut::<&Camera>(player)
            .unwrap()
            .target()
            .as_ref()
            .unwrap()
            .color_tex();
        let material = assets.add_postprocess_material(&state.renderer, pp_src_tex);
        let postprocessor = world.spawn((
            Transform::default(),
            Camera::new(1.0, RENDER_TAG_POST_PROCESS | RENDER_TAG_DEBUG_UI, None),
            Mesh(assets.quad_mesh),
            Material(material),
            RenderOrder(100),
            RenderTags(RENDER_TAG_POST_PROCESS),
        ));

        let mut scene = Self {
            world,
            physics,
            player,
            postprocessor,
            ui: Ui::new(state),
            spawned_startup_box: false,
        };

        scene.spawn_floor(&state.renderer, assets);

        scene
    }

    pub fn handle_event(&mut self, event: &Event<()>, state: &State) {
        self.ui.handle_event(event, state);
    }

    pub fn update(
        &mut self,
        dt: f32,
        state: &State,
        assets: &mut Assets,
        new_canvas_size: &Option<SurfaceSize>,
    ) {
        self.physics.update(dt);

        Player::update(
            dt,
            &mut self.world,
            &mut self.physics,
            &state.input,
            &state.window,
        );
        Grab::update(&mut self.world, &state.input, &mut self.physics);
        PlayerTarget::update(&mut self.world);

        if state.input.action_activated(InputAction::Spawn) || !self.spawned_startup_box {
            let player_transform = self.world.query_one_mut::<&Transform>(self.player).unwrap();
            let pos = if self.spawned_startup_box {
                player_transform.position() + player_transform.forward().xyz() * 5.0
            } else {
                self.spawned_startup_box = true;
                Vec3::y_axis().xyz() * 5.0
            };
            self.spawn_box(pos, Vec3::from_element(1.0), &state.renderer, assets);
        }

        self.sync_physics();

        if let Some(new_size) = new_canvas_size {
            self.resize(new_size, state, assets);
        }

        self.ui.prepare_frame(dt, state, |frame| {
            let window = frame.window("Info");
            window
                .always_auto_resize(true)
                .size([300.0, 150.0], Condition::FirstUseEver)
                .position([20.0, 20.0], Condition::FirstUseEver)
                .build(|| {
                    frame.text("Controls:");
                    frame.text("Tab: capture/release mouse");
                    frame.text("WASDQE: move camera while mouse is captured");
                    frame.text("F: spawn a box");
                    frame.text("Left mouse click: grab/release an object");
                    frame.separator();
                    let mouse_pos = frame.io().mouse_pos;
                    frame.text(format!(
                        "Mouse position: ({:.1},{:.1})",
                        mouse_pos[0], mouse_pos[1]
                    ));
                    frame.text(format!("Frame time: {dt:?}"));
                });
        })
    }

    pub fn render(&mut self, rr: &Renderer, assets: &mut Assets) {
        self.render_with_camera(self.player, rr, assets);
        self.render_with_camera(self.postprocessor, rr, assets);
    }

    fn resize(&mut self, new_size: &SurfaceSize, state: &State, assets: &mut Assets) {
        let mut player_cam = self.world.get::<&mut Camera>(self.player).unwrap();
        player_cam.set_aspect(new_size.width as f32 / new_size.height as f32);
        player_cam
            .target_mut()
            .unwrap()
            .resize((new_size.width, new_size.height), &state.renderer);

        let color_tex = player_cam.target().as_ref().unwrap().color_tex();
        let mut material = self.world.get::<&mut Material>(self.postprocessor).unwrap();
        assets.remove_material(material.0);
        material.0 = assets.add_postprocess_material(&state.renderer, color_tex);
    }

    fn spawn_floor(&mut self, rr: &Renderer, assets: &mut Assets) {
        let pos = Vec3::from_element(0.0);
        let scale = Vec3::new(10.0, 0.5, 10.0);
        let body = RigidBody::cuboid(
            RigidBodyParams {
                pos,
                scale,
                movable: false,
            },
            &mut self.physics,
        );
        let material = assets.add_textured_material(rr, assets.bricks_texture);
        self.world.spawn((
            Transform::new(pos, scale),
            Mesh(assets.box_mesh),
            Material(material),
            body,
            RenderOrder(0),
            RenderTags(RENDER_TAG_SCENE),
        ));
    }

    fn spawn_box(&mut self, pos: Vec3, scale: Vec3, rr: &Renderer, assets: &mut Assets) {
        let body = RigidBody::cuboid(
            RigidBodyParams {
                pos,
                scale,
                movable: true,
            },
            &mut self.physics,
        );
        let material = assets.add_textured_material(rr, assets.crate_texture);
        self.world.spawn((
            Transform::new(pos, scale),
            Mesh(assets.box_mesh),
            Material(material),
            body,
            RenderOrder(0),
            RenderTags(RENDER_TAG_SCENE),
        ));
    }

    fn render_with_camera(&mut self, camera: Entity, rr: &Renderer, assets: &mut Assets) {
        if let Some((cam, cam_tr)) = self
            .world
            .query_one::<(&Camera, &Transform)>(camera)
            .unwrap()
            .get()
        {
            let mut renderables =
                self.world
                    .query::<(&Mesh, &Material, &Transform, &RenderOrder, &RenderTags)>();

            // Pick what should be rendered by the camera
            let mut meshes = renderables
                .iter()
                .filter(|(_, (.., tag))| cam.should_render(tag.0))
                .map(|(_, (mesh, material, transform, order, _))| {
                    (mesh, material, transform, order)
                })
                .collect::<Vec<_>>();

            // Sort by render order
            meshes.sort_by(|&(.., o1), &(.., o2)| o1.0.partial_cmp(&o2.0).unwrap());

            let bundles = meshes
                .into_iter()
                .map(|(mesh, material, transform, _)| {
                    match assets.material_mut(material.0) {
                        materials::Material::Color(m) => m.set_wvp(rr, cam, cam_tr, transform),
                        materials::Material::Skybox(m) => m.set_wvp(rr, cam, cam_tr),
                        materials::Material::Textured(m) => m.set_wvp(rr, cam, cam_tr, transform),
                        materials::Material::PostProcess(_) => (),
                    }
                    rr.build_render_bundle(mesh.0, material.0, cam.target().as_ref(), assets)
                })
                // TODO Avoid vec allocation
                .collect::<Vec<wgpu::RenderBundle>>();

            rr.render_pass(
                &bundles,
                cam.target().as_ref(),
                cam.target().is_none().then_some(&mut self.ui),
            );
        }
    }

    fn sync_physics(&mut self) {
        for (_, (t, b)) in self.world.query_mut::<(&mut Transform, &RigidBody)>() {
            let body = self.physics.body(b.handle());
            t.set(*body.translation(), *body.rotation().inverse().quaternion());
        }
    }
}
