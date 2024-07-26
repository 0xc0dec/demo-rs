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
    // TODO Implement via components, like everything else
    player: Player,
    pp_cam: Camera,
    player_target: Entity,
    pp: Entity,
    grabbed_body: Option<Entity>,
    grabbed_body_player_local_pos: Option<Vec3>,
    spawned_demo_box: bool,
}

impl Scene {
    pub fn new(gfx: &Graphics, assets: &Assets) -> Self {
        let mut physics = Physics::new();
        let player = Player::new(gfx, &mut physics);
        let pp_cam = Camera::new(1.0, RENDER_TAG_POST_PROCESS | RENDER_TAG_DEBUG_UI, None);

        let mut scene = Self {
            world: World::new(),
            materials: SlotMap::new(),
            physics,
            player,
            pp_cam,
            player_target: Entity::DANGLING,
            pp: Entity::DANGLING,
            grabbed_body: None,
            grabbed_body_player_local_pos: None,
            spawned_demo_box: false,
        };

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

        let pp_src_tex = scene.player.camera().target().as_ref().unwrap().color_tex();
        let mat_id = scene
            .materials
            .insert(Box::new(PostProcessMaterial::new(gfx, assets, pp_src_tex)));
        scene.pp = scene.world.spawn((
            Transform::default(),
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

        self.player
            .update(gfx, dt, input, window, &mut self.physics, new_canvas_size);
        self.update_grabbed(input);
        self.update_player_target();

        if input.action_activated(InputAction::Spawn) || !self.spawned_demo_box {
            let pos = if self.spawned_demo_box {
                self.player.transform().position() + self.player.transform().forward().xyz() * 5.0
            } else {
                self.spawned_demo_box = true;
                Vec3::y_axis().xyz() * 5.0
            };
            self.spawn_cube(pos, Vec3::from_element(1.0), gfx, assets);
        }

        self.sync_physics();

        // TODO Should this be inside `render()`? Same for the player updating its RT.
        if new_canvas_size.is_some() {
            // Note: this seems to be relying on player having already updated its render target size.
            let color_tex = self.player.camera().target().as_ref().unwrap().color_tex();
            let mat_id = self.world.query_one_mut::<&MaterialCmp>(self.pp).unwrap().0;
            self.materials[mat_id] = Box::new(PostProcessMaterial::new(gfx, assets, color_tex));
        }
    }

    pub fn render(&mut self, gfx: &Graphics, assets: &Assets) {
        gfx.render_pass(
            &self.build_render_bundles(false, gfx, assets),
            self.player.camera().target().as_ref(),
        );
        gfx.render_pass(&self.build_render_bundles(true, gfx, assets), None);
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

    fn spawn_cube(&mut self, pos: Vec3, scale: Vec3, gfx: &Graphics, assets: &Assets) {
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

    fn update_grabbed(&mut self, input: &Input) {
        if input.action_active(InputAction::Grab) && self.player.controlled() {
            if self.grabbed_body_player_local_pos.is_none() {
                // Initiate grab
                if let Some(focus_body_handle) = self.player.focus_body() {
                    let (body_entity, body) = self
                        .world
                        .query_mut::<&PhysicalBody>()
                        .into_iter()
                        .find(|(_, body)| body.body_handle() == focus_body_handle)
                        .unwrap();
                    body.set_kinematic(&mut self.physics, true);
                    let body = self.physics.bodies.get_mut(focus_body_handle).unwrap();
                    let local_pos = self
                        .player
                        .transform()
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
                    let body = self
                        .world
                        .query_one_mut::<&PhysicalBody>(grabbed_body)
                        .unwrap();
                    let body = self.physics.bodies.get_mut(body.body_handle()).unwrap();
                    let new_pos =
                        self.player.transform().matrix().transform_point(&to_point(
                            self.grabbed_body_player_local_pos.unwrap(),
                        ));
                    body.set_translation(new_pos.coords, true);
                }
            }
        } else {
            // Release grab
            if let Some(grabbed_body) = self.grabbed_body.take() {
                let body = self
                    .world
                    .query_one_mut::<&PhysicalBody>(grabbed_body)
                    .unwrap();
                body.set_kinematic(&mut self.physics, false);
                self.grabbed_body = None;
                self.grabbed_body_player_local_pos = None;
            }
        }
    }

    fn update_player_target(&mut self) {
        let new_tag = if let Some(player_focus_pt) = self.player.focus_point() {
            let dist_to_camera = (self.player.transform().position() - player_focus_pt).magnitude();
            let scale = (dist_to_camera / 10.0).clamp(0.01, 0.1);
            let mut target_transform = self
                .world
                .get::<&mut Transform>(self.player_target)
                .unwrap();
            target_transform.set_position(player_focus_pt);
            target_transform.set_scale(Vec3::from_element(scale));

            RENDER_TAG_SCENE
        } else {
            RENDER_TAG_HIDDEN
        };

        self.world
            .insert_one(self.player_target, RenderTagCmp(new_tag))
            .unwrap();
    }

    fn build_render_bundles(
        &mut self,
        pp: bool,
        gfx: &Graphics,
        assets: &Assets,
    ) -> Vec<wgpu::RenderBundle> {
        let (camera, camera_transform) = if pp {
            (&self.pp_cam, Transform::default())
        } else {
            (self.player.camera(), *self.player.transform())
        };

        let mut renderables = self
            .world
            .query_mut::<(
                &MeshCmp,
                &MaterialCmp,
                &Transform,
                &RenderOrderCmp,
                &RenderTagCmp,
            )>()
            .into_iter()
            .filter(|(_, (.., tag))| camera.should_render(tag.0))
            .map(|(_, (mesh, material, transform, order, _))| (mesh, material, transform, order))
            .collect::<Vec<_>>();
        renderables.sort_by(|&(.., o1), &(.., o2)| o1.0.partial_cmp(&o2.0).unwrap());
        renderables
            .into_iter()
            .map(|(mesh, material, transform, _)| {
                let material = self.materials.get_mut(material.0).unwrap().as_mut();
                let mesh = assets.mesh(mesh.0);
                gfx.build_render_bundle(mesh, material, transform, (camera, &camera_transform))
            })
            .collect::<_>()
    }

    fn sync_physics(&mut self) {
        for (_, (t, b)) in self.world.query_mut::<(&mut Transform, &PhysicalBody)>() {
            let body = self.physics.bodies.get(b.body_handle()).unwrap();
            t.set(*body.translation(), *body.rotation().inverse().quaternion());
        }
    }
}
