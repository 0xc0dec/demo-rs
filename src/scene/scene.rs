use hecs::{Entity, World};
use rapier3d::dynamics::{RigidBodyBuilder, RigidBodyType};
use rapier3d::prelude::*;
use std::collections::HashMap;

use crate::input::InputAction;
use crate::math::Vec3;
use crate::physics::Physics;
use crate::render;
use crate::render::{Renderer, SurfaceSize, Ui};
use crate::scene::scene_config::{ComponentCfg, MaterialCfg, MeshPrefabCfg, SceneCfg};
use crate::state::State;

use super::assets::Assets;
use super::components;
use super::components::{
    Camera, Grab, Hud, Material, Mesh, Player, PlayerTarget, RenderOrder, RenderTags,
    Transform, RENDER_TAG_SCENE,
};

pub struct Scene {
    world: World,
    physics: Physics,
    postprocessor: Entity,
    player: Entity,
    hud: Entity,
    ui: Ui,
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
            Camera::new(
                1.0,
                components::RENDER_TAG_POST_PROCESS | components::RENDER_TAG_DEBUG_UI,
                None,
            ),
            Mesh(assets.quad_mesh),
            Material(material),
            RenderOrder(100),
            RenderTags(components::RENDER_TAG_POST_PROCESS),
        ));

        let hud = world.spawn((Hud,));

        Self {
            world,
            physics,
            player,
            postprocessor,
            hud,
            ui: Ui::new(&state.window, &state.renderer),
        }
    }

    pub fn update(
        &mut self,
        dt: f32,
        state: &State,
        assets: &mut Assets,
        new_canvas_size: &Option<SurfaceSize>,
    ) {
        for e in state.input.new_raw_events() {
            self.ui.handle_event(e, &state.window);
        }

        self.physics.update(dt);

        Player::update(dt, &mut self.world, &mut self.physics, state);
        Grab::update(&mut self.world, &state.input, &mut self.physics);
        PlayerTarget::update(&mut self.world);

        if state.input.action_activated(InputAction::Spawn) {
            let player_tr = self.world.query_one_mut::<&Transform>(self.player).unwrap();
            let pos = player_tr.position() + player_tr.forward().xyz() * 5.0;
            self.spawn_box(pos, Vec3::from_element(1.0), &state.renderer, assets);
        }

        self.sync_physics();

        if let Some(new_size) = new_canvas_size {
            self.resize(new_size, state, assets);
        }

        self.world
            .query_one_mut::<&mut Hud>(self.hud)
            .unwrap()
            .build(dt, state, &mut self.ui);
    }

    pub fn render(&mut self, rr: &Renderer, assets: &Assets) {
        self.render_with_camera(self.player, rr, assets);
        self.render_with_camera(self.postprocessor, rr, assets);
    }

    // TODO Continue adding other stuff until all scene initialization is done via the file.
    pub fn insert_from_cfg(&mut self, cfg: &SceneCfg, state: &State, assets: &mut Assets) {
        let mut materials = HashMap::new();
        for mat in &cfg.materials {
            match mat {
                MaterialCfg::Color {
                    name,
                    color: [r, g, b],
                    wireframe,
                } => {
                    materials.insert(
                        name.clone(),
                        assets.add_color_material(
                            &state.renderer,
                            Vec3::new(*r, *g, *b),
                            wireframe.unwrap_or(false),
                        ),
                    );
                }
                MaterialCfg::Textured { name, texture } => {
                    let tex = assets.add_texture_2d_from_file(&state.renderer, texture);
                    materials.insert(
                        name.clone(),
                        assets.add_textured_material(&state.renderer, tex),
                    );
                }
            }
        }

        for node in cfg.nodes.values() {
            let pos = node
                .pos
                .map(|pos| Vec3::from_row_slice(&pos))
                .unwrap_or(Vec3::zeros());
            let scale = node
                .scale
                .map(|scale| Vec3::from_row_slice(&scale))
                .unwrap_or(Vec3::from_element(1.0));
            let e = self.world.spawn((
                Transform::new(pos, scale),
                RenderOrder(node.render_order),
                RenderTags(node.render_tags),
            ));

            if let Some(body_def) = &node.body {
                let movable = body_def.movable.unwrap_or(true);
                let body_type = if movable {
                    RigidBodyType::Dynamic
                } else {
                    RigidBodyType::Fixed
                };
                // TODO Move this logic into the RigidBody cmp
                let body = RigidBodyBuilder::new(body_type).translation(pos).build();
                // TODO
                let collider = ColliderBuilder::cuboid(scale.x, scale.y, scale.z)
                    .restitution(0.2)
                    .friction(0.7)
                    .build();
                let body = self.physics.add_body(body, collider);
                self.world
                    .insert(
                        e,
                        (components::RigidBody {
                            handle: body,
                            movable,
                        },),
                    )
                    .unwrap();
            }

            if let Some(mesh) = &node.mesh {
                // TODO Cache, look up if already loaded
                let mesh = if let Some(path) = &mesh.path {
                    assets.add_mesh_from_file(&state.renderer, path)
                } else if let Some(prefab) = &mesh.prefab {
                    let mesh = match prefab {
                        MeshPrefabCfg::Basis => render::Mesh::new_basis(&state.renderer),
                    };
                    assets.add_mesh(mesh)
                } else {
                    panic!("Unable to create mesh");
                };
                self.world.insert(e, (Mesh(mesh),)).unwrap();
            }

            for cmp in &node.components {
                match cmp {
                    ComponentCfg::Material { name } => {
                        self.world.insert(e, (Material(materials[name]),)).unwrap();
                    }
                    ComponentCfg::PlayerTarget => {
                        self.world.insert(e, (PlayerTarget,)).unwrap();
                    }
                }
            }
        }
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

    fn spawn_box(&mut self, pos: Vec3, scale: Vec3, rr: &Renderer, assets: &mut Assets) {
        let body = components::RigidBody::cuboid(
            components::RigidBodyParams {
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

    fn render_with_camera(&mut self, camera: Entity, rr: &Renderer, assets: &Assets) {
        if let Some((cam, cam_tr)) = self
            .world
            .query_one::<(&Camera, &Transform)>(camera)
            .unwrap()
            .get()
        {
            let mut items = self
                .world
                .query::<(&Mesh, &Material, &Transform, &RenderOrder, &RenderTags)>();

            // Pick what should be rendered by the camera
            let mut items = items
                .iter()
                .filter(|(_, (.., tag))| cam.should_render(tag.0))
                .map(|(_, (mesh, material, transform, order, _))| {
                    (mesh, material, transform, order)
                })
                .collect::<Vec<_>>();

            // Sort by render order
            items.sort_by(|&(.., o1), &(.., o2)| o1.0.partial_cmp(&o2.0).unwrap());

            let bundles = items
                .into_iter()
                .map(|(mesh, mat, tr, _)| {
                    let mat = assets.material(mat.0);
                    mat.update(rr, cam, cam_tr, tr);
                    rr.build_render_bundle(assets.mesh(mesh.0), mat, cam.target().as_ref())
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
        for (_, (t, b)) in self
            .world
            .query_mut::<(&mut Transform, &components::RigidBody)>()
        {
            let body = self.physics.body(b.handle());
            t.set(*body.translation(), *body.rotation().inverse().quaternion());
        }
    }
}
