use hecs::{Entity, World};
use std::collections::HashMap;

use crate::input::InputAction;
use crate::math::Vec3;
use crate::physics::Physics;
use crate::render::{Renderer, SurfaceSize, Ui};
use crate::scene::scene_definition::{ComponentDef, MaterialDef, SceneDef};
use crate::state::State;

use super::assets::Assets;
use super::components::{
    Camera, Grab, Hud, Material, Mesh, Player, PlayerTarget, RenderOrder, RenderTags,
    Transform, RENDER_TAG_SCENE,
};
use super::{components, materials};

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

        let mut scene = Self {
            world,
            physics,
            player,
            postprocessor,
            hud,
            ui: Ui::new(&state.window, &state.renderer),
        };

        scene.spawn_floor(&state.renderer, assets);
        scene.spawn_box(
            Vec3::y_axis().xyz() * 5.0,
            Vec3::from_element(1.0),
            &state.renderer,
            assets,
        );
        scene.spawn_basis(&state.renderer, assets);

        // TODO
        let def_file_content = String::from_utf8_lossy(include_bytes!("../../assets/scene.yml"));
        scene.insert_from_definition(
            &SceneDef::from_yaml(&def_file_content),
            &state.renderer,
            assets,
        );

        scene
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
            let player_transform = self.world.query_one_mut::<&Transform>(self.player).unwrap();
            let pos = player_transform.position() + player_transform.forward().xyz() * 5.0;
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

    fn insert_from_definition(&mut self, def: &SceneDef, rr: &Renderer, assets: &mut Assets) {
        let mut materials = HashMap::new();
        for mat in &def.materials {
            match mat {
                MaterialDef::Color {
                    name,
                    color: [r, g, b],
                    wireframe,
                } => {
                    materials.insert(
                        name.clone(),
                        // TODO Color
                        assets.add_color_material(rr, wireframe.unwrap_or(false)),
                    );
                }
                MaterialDef::Textured { name, texture } => {
                    materials.insert(
                        name.clone(),
                        // TODO Texture
                        assets.add_textured_material(rr, assets.bricks_texture),
                    );
                }
            }
        }

        let mut meshes = HashMap::new();

        for node in def.nodes.values() {
            let e = self
                .world
                .spawn((RenderOrder(node.render_order), RenderTags(node.render_tags)));
            for cmp in &node.components {
                match cmp {
                    ComponentDef::Transform { pos, scale } => {
                        self.world
                            .insert(
                                e,
                                (Transform::new(
                                    Vec3::from_row_slice(pos),
                                    Vec3::from_row_slice(scale),
                                ),),
                            )
                            .unwrap();
                    }
                    ComponentDef::Mesh { path } => {
                        if !meshes.contains_key(path) {
                            meshes.insert(path, assets.add_mesh(rr, path));
                        }
                        self.world.insert(e, (Mesh(meshes[path]),)).unwrap();
                    }
                    ComponentDef::Material { name } => {
                        self.world.insert(e, (Material(materials[name]),)).unwrap();
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

    fn spawn_floor(&mut self, rr: &Renderer, assets: &mut Assets) {
        let pos = Vec3::from_element(0.0);
        let scale = Vec3::new(10.0, 0.5, 10.0);
        let body = components::RigidBody::cuboid(
            components::RigidBodyParams {
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

    fn spawn_basis(&mut self, rr: &Renderer, assets: &mut Assets) {
        let mat = assets.add_color_material(rr, true);
        if let materials::Material::Color(m) = assets.material(mat) {
            m.set_color(rr, Vec3::new(1.0, 1.0, 0.0))
        }

        let mut t = Transform::default();
        t.translate(Vec3::new(2.0, 1.0, 2.0));

        self.world.spawn((
            t,
            Mesh(assets.basis_mesh),
            Material(mat),
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
