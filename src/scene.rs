use hecs::{Entity, World};
use slotmap::{DefaultKey, SlotMap};
use winit::window::Window;

use crate::assets::{Assets, MeshId};
use crate::camera::Camera;
use crate::graphics::{Graphics, SurfaceSize};
use crate::input::{Input, InputAction};
use crate::materials::{
    ColorMaterial, DiffuseMaterial, Material, PostProcessMaterial, SkyboxMaterial,
};
use crate::math::{to_point, Vec3};
use crate::physical_body::{PhysicalBody, PhysicalBodyParams};
use crate::physics::Physics;
use crate::player::Player;
use crate::render_tags::{
    RENDER_TAG_DEBUG_UI, RENDER_TAG_HIDDEN, RENDER_TAG_POST_PROCESS, RENDER_TAG_SCENE,
};
use crate::transform::Transform;

// TODO Remove `Cmp` suffix?
struct MeshCmp(MeshId);
struct MaterialCmp(MaterialId);
struct RenderOrderCmp(i32);
struct RenderTagCmp(u32);

type MaterialId = DefaultKey;

pub struct Scene {
    world: World,
    physics: Physics,
    materials: SlotMap<MaterialId, Box<dyn Material>>,
    postprocessor: Entity,
    player: Entity,
    // TODO Store in Player?
    player_target: Entity,
    grabbed_body: Option<Entity>,
    grabbed_body_player_local_pos: Option<Vec3>,
    spawned_box_at_startup: bool,
}

impl Scene {
    pub fn new(gfx: &Graphics, assets: &Assets) -> Self {
        let mut scene = Self {
            world: World::new(),
            materials: SlotMap::new(),
            physics: Physics::new(),
            player: Entity::DANGLING,
            player_target: Entity::DANGLING,
            postprocessor: Entity::DANGLING,
            grabbed_body: None,
            grabbed_body_player_local_pos: None,
            spawned_box_at_startup: false,
        };

        // Player
        scene.player = Player::spawn(
            &mut scene.world,
            gfx,
            &mut scene.physics,
            Vec3::new(7.0, 7.0, 7.0),
        );

        // Player target
        let mat_id = scene
            .materials
            .insert(Box::new(ColorMaterial::new(gfx, assets)));
        scene.player_target = scene.world.spawn((
            Transform::default(),
            MeshCmp(assets.box_mesh_id()),
            MaterialCmp(mat_id),
            RenderOrderCmp(0),
            RenderTagCmp(RENDER_TAG_HIDDEN),
        ));

        // Floor
        scene.spawn_floor(gfx, assets);

        // Skybox
        // Spawning skybox somewhere in the middle to ensure the sorting by render order works and it still shows up
        // in the background.
        let mat_id = scene.materials.insert(Box::new(SkyboxMaterial::new(
            gfx,
            assets,
            assets.skybox_texture(),
        )));
        scene.world.spawn((
            Transform::default(),
            MeshCmp(assets.quad_mesh_id()),
            MaterialCmp(mat_id),
            RenderOrderCmp(-100),
            RenderTagCmp(RENDER_TAG_SCENE),
        ));

        // Post-processor
        let pp_src_tex = scene
            .world
            .query_one_mut::<&Camera>(scene.player)
            .unwrap()
            .target()
            .as_ref()
            .unwrap()
            .color_tex();
        let mat_id = scene
            .materials
            .insert(Box::new(PostProcessMaterial::new(gfx, assets, pp_src_tex)));
        scene.postprocessor = scene.world.spawn((
            Transform::default(),
            Camera::new(1.0, RENDER_TAG_POST_PROCESS | RENDER_TAG_DEBUG_UI, None),
            MeshCmp(assets.quad_mesh_id()),
            MaterialCmp(mat_id),
            RenderOrderCmp(100),
            RenderTagCmp(RENDER_TAG_POST_PROCESS),
        ));

        scene
    }

    fn new_diffuse_mat(&mut self, gfx: &Graphics, assets: &Assets) -> MaterialId {
        self.materials.insert(Box::new(DiffuseMaterial::new(
            gfx,
            assets,
            assets.stone_texture(),
        )))
    }

    pub fn update(
        &mut self,
        dt: f32,
        gfx: &Graphics,
        input: &Input,
        window: &Window,
        assets: &Assets,
        new_canvas_size: &Option<SurfaceSize>,
    ) {
        self.physics.update(dt);

        Player::update(dt, &mut self.world, &mut self.physics, input, window);
        self.update_grabbed(input);
        self.update_player_target();

        if input.action_activated(InputAction::Spawn) || !self.spawned_box_at_startup {
            let player_transform = self.world.query_one_mut::<&Transform>(self.player).unwrap();
            let pos = if self.spawned_box_at_startup {
                player_transform.position() + player_transform.forward().xyz() * 5.0
            } else {
                self.spawned_box_at_startup = true;
                Vec3::y_axis().xyz() * 5.0
            };
            self.spawn_box(pos, Vec3::from_element(1.0), gfx, assets);
        }

        self.sync_physics();

        if let Some(new_size) = new_canvas_size {
            self.handle_canvas_resize(new_size, gfx, assets);
        }
    }

    pub fn render(&mut self, gfx: &Graphics, assets: &Assets) {
        self.render_camera(self.player, gfx, assets);
        self.render_camera(self.postprocessor, gfx, assets);
    }

    fn handle_canvas_resize(&mut self, new_size: &SurfaceSize, gfx: &Graphics, assets: &Assets) {
        let mut player_cam = self.world.get::<&mut Camera>(self.player).unwrap();
        player_cam.set_aspect(new_size.width as f32 / new_size.height as f32);
        player_cam
            .target_mut()
            .unwrap()
            .resize((new_size.width, new_size.height), gfx);

        let color_tex = player_cam.target().as_ref().unwrap().color_tex();
        let mat_id = self
            .world
            .get::<&MaterialCmp>(self.postprocessor)
            .unwrap()
            .0;
        self.materials[mat_id] = Box::new(PostProcessMaterial::new(gfx, assets, color_tex));
    }

    fn spawn_floor(&mut self, gfx: &Graphics, assets: &Assets) {
        let pos = Vec3::from_element(0.0);
        let scale = Vec3::new(10.0, 0.5, 10.0);
        let body = PhysicalBody::cuboid(
            PhysicalBodyParams {
                pos,
                scale,
                movable: false,
            },
            &mut self.physics,
        );

        let mat_id = self.new_diffuse_mat(gfx, assets);
        self.world.spawn((
            Transform::new(pos, scale),
            MeshCmp(assets.box_mesh_id()),
            MaterialCmp(mat_id),
            body,
            RenderOrderCmp(0),
            RenderTagCmp(RENDER_TAG_SCENE),
        ));
    }

    fn spawn_box(&mut self, pos: Vec3, scale: Vec3, gfx: &Graphics, assets: &Assets) {
        let body = PhysicalBody::cuboid(
            PhysicalBodyParams {
                pos,
                scale,
                movable: true,
            },
            &mut self.physics,
        );
        let mat_id = self.new_diffuse_mat(gfx, assets);
        self.world.spawn((
            Transform::new(pos, scale),
            MeshCmp(assets.box_mesh_id()),
            MaterialCmp(mat_id),
            body,
            RenderOrderCmp(0),
            RenderTagCmp(RENDER_TAG_SCENE),
        ));
    }

    // TODO Extract into a separate component
    fn update_grabbed(&mut self, input: &Input) {
        let mut q = self
            .world
            .query_one::<(&Player, &Transform)>(self.player)
            .unwrap();
        let (player, player_tr) = q.get().unwrap();
        if input.action_active(InputAction::Grab) && player.controlled() {
            if self.grabbed_body_player_local_pos.is_none() {
                // Initiate grab
                if let Some(look_at_body) = player.look_at_body() {
                    let mut q = self.world.query::<&PhysicalBody>();
                    let (body_entity, body) = q
                        .into_iter()
                        .find(|(_, body)| body.body_handle() == look_at_body)
                        .unwrap();
                    body.set_kinematic(&mut self.physics, true);
                    let body = self.physics.bodies.get_mut(look_at_body).unwrap();
                    let local_pos = player_tr
                        .matrix()
                        .try_inverse()
                        .unwrap()
                        .transform_point(&to_point(*body.translation()))
                        .coords;
                    self.grabbed_body = Some(body_entity);
                    self.grabbed_body_player_local_pos = Some(local_pos);
                }
            } else {
                // Update the grabbed object
                if let Some(grabbed_body) = self.grabbed_body {
                    let new_pos = player_tr
                        .matrix()
                        .transform_point(&to_point(self.grabbed_body_player_local_pos.unwrap()));
                    let body = self
                        .world
                        .query_one::<&PhysicalBody>(grabbed_body)
                        .unwrap()
                        .get()
                        .unwrap()
                        .body_handle();
                    self.physics
                        .bodies
                        .get_mut(body)
                        .unwrap()
                        .set_translation(new_pos.coords, true);
                }
            }
        } else {
            // Release grab
            if let Some(grabbed_body) = self.grabbed_body.take() {
                self.world
                    .query_one::<&PhysicalBody>(grabbed_body)
                    .unwrap()
                    .get()
                    .unwrap()
                    .set_kinematic(&mut self.physics, false);
                self.grabbed_body = None;
                self.grabbed_body_player_local_pos = None;
            }
        }
    }

    fn update_player_target(&mut self) {
        let (player, player_tr) = self
            .world
            .query_one_mut::<(&Player, &Transform)>(self.player)
            .unwrap();
        let new_tag = if let Some(look_at_pt) = player.look_at_point() {
            let dist_to_camera = (player_tr.position() - look_at_pt).magnitude();
            let scale = (dist_to_camera / 10.0).clamp(0.01, 0.1);
            let mut target_transform = self
                .world
                .get::<&mut Transform>(self.player_target)
                .unwrap();
            target_transform.set_position(look_at_pt);
            target_transform.set_scale(Vec3::from_element(scale));
            RENDER_TAG_SCENE
        } else {
            RENDER_TAG_HIDDEN
        };

        self.world
            .insert_one(self.player_target, RenderTagCmp(new_tag))
            .unwrap();
    }

    fn render_camera(&mut self, camera: Entity, gfx: &Graphics, assets: &Assets) {
        for (_, (camera, camera_transform)) in &mut self
            .world
            .query::<(&Camera, &Transform)>()
            .iter()
            // TODO This is a workaround to always render via a single camera. We should iterate over all cameras
            // based on their render order or smth similar.
            .filter(|(cam_ent, _)| *cam_ent == camera)
        {
            let mut renderables = self.world.query::<(
                &MeshCmp,
                &MaterialCmp,
                &Transform,
                &RenderOrderCmp,
                &RenderTagCmp,
            )>();
            let mut renderables = renderables
                .iter()
                .filter(|(_, (.., tag))| camera.should_render(tag.0))
                .map(|(_, (mesh, material, transform, order, _))| {
                    (mesh, material, transform, order)
                })
                .collect::<Vec<_>>();
            renderables.sort_by(|&(.., o1), &(.., o2)| o1.0.partial_cmp(&o2.0).unwrap());
            let bundles = renderables
                .into_iter()
                .map(|(mesh, material, transform, _)| {
                    let material = self.materials.get_mut(material.0).unwrap().as_mut();
                    let mesh = assets.mesh(mesh.0);
                    gfx.build_render_bundle(mesh, material, transform, (camera, camera_transform))
                })
                .collect::<Vec<wgpu::RenderBundle>>();
            gfx.render_pass(&bundles, camera.target().as_ref());
        }
    }

    fn sync_physics(&mut self) {
        for (_, (t, b)) in self.world.query_mut::<(&mut Transform, &PhysicalBody)>() {
            let body = self.physics.bodies.get(b.body_handle()).unwrap();
            t.set(*body.translation(), *body.rotation().inverse().quaternion());
        }
    }
}
