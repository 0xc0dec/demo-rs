use std::ops::DerefMut;
use std::rc::Rc;

use wgpu::RenderBundle;

use crate::assets::Assets;
use crate::camera::Camera;
use crate::graphics::Graphics;
use crate::input::{Input, InputAction};
use crate::materials::{
    ColorMaterial, DiffuseMaterial, Material, PostProcessMaterial, SkyboxMaterial,
};
use crate::math::{to_point, Vec3};
use crate::mesh::Mesh;
use crate::physical_body::{PhysicalBody, PhysicalBodyParams};
use crate::physics::Physics;
use crate::player::Player;
use crate::render::build_render_bundle;
use crate::render_tags::{RENDER_TAG_HIDDEN, RENDER_TAG_POST_PROCESS, RENDER_TAG_SCENE};
use crate::texture::Texture;
use crate::transform::Transform;

// TODO A proper ECS or some other solution. This is a very basic solution for now.
// TODO Use arena.
pub struct Scene {
    physics: Physics,
    transforms: Vec<Option<Transform>>,
    meshes: Vec<Option<Rc<Mesh>>>,
    materials: Vec<Option<Box<dyn Material>>>,
    bodies: Vec<Option<PhysicalBody>>,
    render_orders: Vec<i32>,
    render_tags: Vec<Option<u32>>,
    grabbed_body_id: Option<usize>,
    grabbed_body_player_local_pos: Option<Vec3>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            physics: Physics::new(),
            transforms: Vec::new(),
            meshes: Vec::new(),
            materials: Vec::new(),
            bodies: Vec::new(),
            render_orders: Vec::new(),
            render_tags: Vec::new(),
            grabbed_body_id: None,
            grabbed_body_player_local_pos: None,
        }
    }

    // TODO Remove
    pub fn physics_mut(&mut self) -> &mut Physics {
        &mut self.physics
    }

    fn spawn_mesh(
        &mut self,
        transform: Transform,
        mesh: Rc<Mesh>,
        material: Box<dyn Material>,
        body: Option<PhysicalBody>,
        render_order: Option<i32>,
        render_tags: Option<u32>,
    ) -> usize {
        self.transforms.push(Some(transform));
        self.meshes.push(Some(mesh));
        self.materials.push(Some(material));
        self.bodies.push(body);
        self.render_orders.push(render_order.unwrap_or(0));
        self.render_tags.push(render_tags);
        self.transforms.len() - 1
    }

    pub fn update(&mut self, dt: f32) {
        self.physics.update(dt);
    }

    pub fn spawn_floor(&mut self, gfx: &Graphics, assets: &Assets) {
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
        self.spawn_mesh(
            Transform::new(pos, scale),
            assets.box_mesh(),
            Box::new(DiffuseMaterial::new(gfx, assets, assets.stone_texture())),
            Some(body),
            None,
            Some(RENDER_TAG_SCENE),
        );
    }

    pub fn spawn_cube(&mut self, pos: Vec3, scale: Vec3, gfx: &Graphics, assets: &Assets) {
        let body = PhysicalBody::cuboid(
            PhysicalBodyParams {
                pos,
                scale,
                movable: true,
            },
            &mut self.physics,
        );
        self.spawn_mesh(
            Transform::new(pos, scale),
            assets.box_mesh(),
            Box::new(DiffuseMaterial::new(gfx, assets, assets.stone_texture())),
            Some(body),
            None,
            Some(RENDER_TAG_SCENE),
        );
    }

    pub fn spawn_skybox(&mut self, gfx: &Graphics, assets: &Assets) {
        self.spawn_mesh(
            Transform::default(),
            Rc::new(Mesh::quad(gfx)),
            Box::new(SkyboxMaterial::new(gfx, assets, assets.skybox_texture())),
            None,
            Some(-100),
            Some(RENDER_TAG_SCENE),
        );
    }

    pub fn spawn_player_target(&mut self, gfx: &Graphics, assets: &Assets) -> usize {
        self.spawn_mesh(
            Transform::default(),
            assets.box_mesh(),
            Box::new(ColorMaterial::new(gfx, assets)),
            None,
            None,
            Some(RENDER_TAG_HIDDEN),
        )
    }

    pub fn spawn_post_process_overlay(
        &mut self,
        source_color_tex: &Texture,
        gfx: &Graphics,
        assets: &Assets,
    ) -> usize {
        self.spawn_mesh(
            Transform::default(),
            Rc::new(Mesh::quad(gfx)),
            Box::new(PostProcessMaterial::new(gfx, assets, source_color_tex)),
            None,
            Some(100),
            Some(RENDER_TAG_POST_PROCESS),
        )
    }

    pub fn update_grabbed(&mut self, player: &Player, input: &Input) {
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
                    body.set_kinematic(&mut self.physics, true);
                    let body = self.physics.bodies.get_mut(focus_body_handle).unwrap();
                    let local_pos = player
                        .transform()
                        .matrix()
                        .try_inverse()
                        .unwrap()
                        .transform_point(&to_point(*body.translation()))
                        .coords;
                    self.grabbed_body_id = Some(body_idx);
                    self.grabbed_body_player_local_pos = Some(local_pos);
                }
            } else {
                // Update the grabbed object
                if let Some(grabbed_idx) = self.grabbed_body_id {
                    let body = self.bodies.get_mut(grabbed_idx).unwrap().as_mut().unwrap();
                    let body = self.physics.bodies.get_mut(body.body_handle()).unwrap();
                    let new_pos = player
                        .transform()
                        .matrix()
                        .transform_point(&to_point(self.grabbed_body_player_local_pos.unwrap()));
                    body.set_translation(new_pos.coords, true);
                }
            }
        } else {
            // Release grab
            if let Some(grabbed_idx) = self.grabbed_body_id.take() {
                let body = self.bodies.get_mut(grabbed_idx).unwrap().as_mut().unwrap();
                body.set_kinematic(&mut self.physics, false);
                self.grabbed_body_id = None;
                self.grabbed_body_player_local_pos = None;
            }
        }
    }

    pub fn update_post_process_overlay(
        &mut self,
        idx: usize,
        source_color_tex: &Texture,
        gfx: &Graphics,
        assets: &Assets,
    ) {
        self.materials[idx] = Some(Box::new(PostProcessMaterial::new(
            gfx,
            assets,
            source_color_tex,
        )));
    }

    pub fn update_player_target(&mut self, player: &Player, target_idx: usize) {
        let new_tag = if let Some(player_focus_pt) = player.focus_point() {
            let dist_to_camera = (player.transform().position() - player_focus_pt).magnitude();
            let scale = (dist_to_camera / 10.0).min(0.1).max(0.01);

            let target_transform = self
                .transforms
                .get_mut(target_idx)
                .unwrap()
                .as_mut()
                .unwrap();
            target_transform.set_position(player_focus_pt);
            target_transform.set_scale(Vec3::from_element(scale));

            RENDER_TAG_SCENE
        } else {
            RENDER_TAG_HIDDEN
        };

        *self
            .render_tags
            .get_mut(target_idx)
            .unwrap()
            .as_mut()
            .unwrap() = new_tag;
    }

    // TODO Move elsewhere, it should not be a method on Scene
    pub fn build_render_bundles(
        &mut self,
        camera: &Camera,
        camera_transform: &Transform,
        gfx: &Graphics,
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
                camera.should_render(self.render_tags.get(idx).unwrap().unwrap_or(0u32))
            })
            .map(|&idx| {
                build_render_bundle(
                    self.meshes.get(idx).unwrap().as_ref().unwrap(),
                    self.materials
                        .get_mut(idx)
                        .unwrap()
                        .as_mut()
                        .unwrap()
                        .deref_mut(),
                    self.transforms.get(idx).unwrap().as_ref().unwrap(),
                    (&camera, &camera_transform),
                    gfx,
                )
            })
            .collect::<Vec<_>>()
    }

    pub fn sync_physics(&mut self) {
        for idx in 0..self.bodies.len() {
            if let Some(body) = self.bodies.get(idx).unwrap() {
                let transform = self.transforms.get_mut(idx).unwrap().as_mut().unwrap();
                let body = self.physics.bodies.get(body.body_handle()).unwrap();
                let phys_pos = body.translation();
                let phys_rot = body.rotation().inverse(); // Not sure why inverse is needed
                transform.set(*phys_pos, *phys_rot.quaternion());
            }
        }
    }
}
