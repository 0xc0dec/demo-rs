use std::ops::DerefMut;
use std::rc::Rc;

use wgpu::RenderBundle;
use winit::window::Window;

use crate::assets::Assets;
use crate::camera::Camera;
use crate::graphics::{Graphics, SurfaceSize};
use crate::input::{Input, InputAction};
use crate::materials::{
    ColorMaterial, DiffuseMaterial, Material, PostProcessMaterial, SkyboxMaterial,
};
use crate::math::{to_point, Vec3};
use crate::mesh::Mesh;
use crate::physical_body::{PhysicalBody, PhysicalBodyParams};
use crate::physics::Physics;
use crate::player::Player;
use crate::render::{build_render_bundle, render_pass};
use crate::render_tags::{
    RENDER_TAG_DEBUG_UI, RENDER_TAG_HIDDEN, RENDER_TAG_POST_PROCESS, RENDER_TAG_SCENE,
};
use crate::transform::Transform;

// TODO A proper ECS or some other solution. This is a very basic solution for now.
// TODO Use arena.
pub struct Scene {
    physics: Physics,
    // TODO Implement via components, like everything else
    player: Player,
    pp_cam: Camera,
    transforms: Vec<Option<Transform>>,
    meshes: Vec<Option<Rc<Mesh>>>,
    materials: Vec<Option<Box<dyn Material>>>,
    bodies: Vec<Option<PhysicalBody>>,
    render_orders: Vec<i32>,
    render_tags: Vec<Option<u32>>,
    player_target_id: usize,
    pp_id: usize,
    grabbed_body_id: Option<usize>,
    grabbed_body_player_local_pos: Option<Vec3>,
    spawned_demo_box: bool,
}

impl Scene {
    pub fn new(gfx: &Graphics, assets: &Assets) -> Self {
        let mut physics = Physics::new();
        let player = Player::new(gfx, &mut physics);
        let pp_cam = Camera::new(1.0, RENDER_TAG_POST_PROCESS | RENDER_TAG_DEBUG_UI, None);

        let mut scene = Self {
            physics,
            player,
            pp_cam,
            transforms: Vec::new(),
            meshes: Vec::new(),
            materials: Vec::new(),
            bodies: Vec::new(),
            render_orders: Vec::new(),
            render_tags: Vec::new(),
            player_target_id: 0,
            pp_id: 0,
            grabbed_body_id: None,
            grabbed_body_player_local_pos: None,
            spawned_demo_box: false,
        };

        scene.player_target_id = scene.spawn_mesh(
            Transform::default(),
            assets.box_mesh(),
            Box::new(ColorMaterial::new(gfx, assets)),
            None,
            None,
            Some(RENDER_TAG_HIDDEN),
        );
        scene.spawn_floor(gfx, assets);
        // Spawning skybox last to ensure the sorting by render order works and it still shows up
        // in the background.
        scene.spawn_skybox(gfx, assets);
        scene.pp_id = scene.spawn_post_process_overlay(gfx, assets);

        scene
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
        self.update_player_target(self.player_target_id);

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

        if new_canvas_size.is_some() {
            let color_tex = self.player.camera().target().as_ref().unwrap().color_tex();
            self.materials[self.pp_id] =
                Some(Box::new(PostProcessMaterial::new(gfx, assets, color_tex)));
        }
    }

    pub fn render(&mut self, gfx: &Graphics) {
        render_pass(
            gfx,
            &self.build_render_bundles(false, gfx),
            self.player.camera().target().as_ref(),
        );
        render_pass(gfx, &self.build_render_bundles(true, gfx), None);
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
        self.spawn_mesh(
            Transform::new(pos, scale),
            assets.box_mesh(),
            Box::new(DiffuseMaterial::new(gfx, assets, assets.stone_texture())),
            Some(body),
            None,
            Some(RENDER_TAG_SCENE),
        );
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
        self.spawn_mesh(
            Transform::new(pos, scale),
            assets.box_mesh(),
            Box::new(DiffuseMaterial::new(gfx, assets, assets.stone_texture())),
            Some(body),
            None,
            Some(RENDER_TAG_SCENE),
        );
    }

    fn spawn_skybox(&mut self, gfx: &Graphics, assets: &Assets) {
        self.spawn_mesh(
            Transform::default(),
            Rc::new(Mesh::quad(gfx)),
            Box::new(SkyboxMaterial::new(gfx, assets, assets.skybox_texture())),
            None,
            Some(-100),
            Some(RENDER_TAG_SCENE),
        );
    }

    fn spawn_post_process_overlay(&mut self, gfx: &Graphics, assets: &Assets) -> usize {
        let source_color_tex = self.player.camera().target().as_ref().unwrap().color_tex();
        self.spawn_mesh(
            Transform::default(),
            Rc::new(Mesh::quad(gfx)),
            Box::new(PostProcessMaterial::new(gfx, assets, source_color_tex)),
            None,
            Some(100),
            Some(RENDER_TAG_POST_PROCESS),
        )
    }

    fn update_grabbed(&mut self, input: &Input) {
        if input.action_active(InputAction::Grab) && self.player.controlled() {
            if self.grabbed_body_player_local_pos.is_none() {
                // Initiate grab
                if let Some(focus_body_handle) = self.player.focus_body() {
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
                    let local_pos = self
                        .player
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
                    let new_pos =
                        self.player.transform().matrix().transform_point(&to_point(
                            self.grabbed_body_player_local_pos.unwrap(),
                        ));
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

    fn update_player_target(&mut self, target_id: usize) {
        let new_tag = if let Some(player_focus_pt) = self.player.focus_point() {
            let dist_to_camera = (self.player.transform().position() - player_focus_pt).magnitude();
            let scale = (dist_to_camera / 10.0).min(0.1).max(0.01);

            let target_transform = self
                .transforms
                .get_mut(target_id)
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
            .get_mut(target_id)
            .unwrap()
            .as_mut()
            .unwrap() = new_tag;
    }

    fn build_render_bundles(&mut self, pp: bool, gfx: &Graphics) -> Vec<RenderBundle> {
        let camera = if pp {
            &self.pp_cam
        } else {
            self.player.camera()
        };
        let camera_transform = if pp {
            Transform::default()
        } else {
            *self.player.transform()
        };

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

    fn sync_physics(&mut self) {
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

#[cfg(test)]
mod tests {
    use std::any::TypeId;
    use std::collections::HashMap;

    use anymap::AnyMap;
    use slotmap::{DefaultKey, SlotMap};

    type SlotKey = DefaultKey;
    type Entity = SlotKey;

    struct TransformCmp(u32);
    struct MeshCmp(u32);

    struct World {
        // TODO Use `DenseSlotMap` where iteration should be fast e.g. in `components`.
        entities: SlotMap<Entity, HashMap<TypeId, SlotKey>>,
        // TODO Use `SecondaryMap`?
        components: AnyMap,
    }

    impl World {
        fn new() -> Self {
            Self {
                entities: SlotMap::new(),
                components: AnyMap::new(),
            }
        }

        fn add_entity(&mut self) -> Entity {
            self.entities.insert(HashMap::new())
        }

        fn set_component<C: 'static>(&mut self, e: Entity, c: C) {
            let key = self.entities[e].entry(TypeId::of::<C>()).or_default();
            let components = self
                .components
                .entry::<SlotMap<_, C>>()
                .or_insert_with(|| SlotMap::new());
            components.remove(*key);
            *key = components.insert(c);
        }

        fn component_mut<C: 'static>(&mut self, e: Entity) -> Option<&mut C> {
            let key = self.entities[e].entry(TypeId::of::<C>()).or_default();
            let components = self
                .components
                .entry::<SlotMap<_, C>>()
                .or_insert_with(|| SlotMap::new());
            components.get_mut(*key)
        }

        fn iter_mut<C: 'static>(&mut self) -> impl Iterator<Item = &mut C> {
            self.components
                .get_mut::<SlotMap<SlotKey, C>>()
                .map(|m| m.iter_mut())
                .unwrap()
                .map(|x| x.1)
        }
    }

    #[test]
    fn smoke() {
        let mut w = World::new();

        let e1 = w.add_entity();
        w.set_component(e1, TransformCmp(1));
        w.set_component(e1, MeshCmp(2));
        assert_eq!(1, w.component_mut::<TransformCmp>(e1).unwrap().0);
        assert_eq!(2, w.component_mut::<MeshCmp>(e1).unwrap().0);

        w.component_mut::<TransformCmp>(e1).unwrap().0 = 11;
        w.component_mut::<MeshCmp>(e1).unwrap().0 = 22;
        assert_eq!(11, w.component_mut::<TransformCmp>(e1).unwrap().0);
        assert_eq!(22, w.component_mut::<MeshCmp>(e1).unwrap().0);

        let e2 = w.add_entity();
        w.set_component(e2, TransformCmp(111));

        for t in w.iter_mut::<TransformCmp>() {
            t.0 = 666;
        }

        assert_eq!(
            &[666, 666],
            &w.iter_mut::<TransformCmp>()
                .map(|t| t.0)
                .collect::<Vec<u32>>()[..]
        );
    }
}
