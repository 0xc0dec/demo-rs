use hecs::{Entity, World};
use winit::window::Window;

use crate::assets::Assets;
use crate::components::{
    Camera, Grab, Material, Mesh, Player, PlayerTarget, RENDER_TAG_DEBUG_UI, RENDER_TAG_POST_PROCESS, RENDER_TAG_SCENE,
    RenderOrder, RenderTags, RigidBody, RigidBodyParams, Transform,
};
use crate::graphics::{Graphics, SurfaceSize};
use crate::input::{Input, InputAction};
use crate::materials;
use crate::math::Vec3;
use crate::physics::Physics;

pub struct Scene {
    world: World,
    physics: Physics,
    postprocessor: Entity,
    player: Entity,
    spawned_startup_box: bool,
}

impl Scene {
    pub fn new(gfx: &Graphics, assets: &mut Assets) -> Self {
        let mut scene = Self {
            world: World::new(),
            physics: Physics::new(),
            player: Entity::DANGLING,
            postprocessor: Entity::DANGLING,
            spawned_startup_box: false,
        };

        // Player
        scene.player = Player::spawn(
            &mut scene.world,
            gfx,
            &mut scene.physics,
            Vec3::new(7.0, 7.0, 7.0),
        );

        // Player target
        PlayerTarget::spawn(gfx, &mut scene.world, assets);

        // Floor
        scene.spawn_floor(gfx, assets);

        // Skybox
        // Spawning skybox somewhere in the middle to ensure the sorting by render order works and it still shows up
        // in the background.
        let material = assets.add_skybox_material(gfx, assets.skybox_texture);
        scene.world.spawn((
            Transform::default(),
            Mesh(assets.quad_mesh),
            Material(material),
            RenderOrder(-100),
            RenderTags(RENDER_TAG_SCENE),
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
        let material = assets.add_postprocess_material(gfx, pp_src_tex);
        scene.postprocessor = scene.world.spawn((
            Transform::default(),
            Camera::new(1.0, RENDER_TAG_POST_PROCESS | RENDER_TAG_DEBUG_UI, None),
            Mesh(assets.quad_mesh),
            Material(material),
            RenderOrder(100),
            RenderTags(RENDER_TAG_POST_PROCESS),
        ));

        scene
    }

    pub fn update(
        &mut self,
        dt: f32,
        gfx: &Graphics,
        input: &Input,
        window: &Window,
        assets: &mut Assets,
        new_canvas_size: &Option<SurfaceSize>,
    ) {
        self.physics.update(dt);

        Player::update(dt, &mut self.world, &mut self.physics, input, window);
        Grab::update(&mut self.world, input, &mut self.physics);
        PlayerTarget::update(&mut self.world);

        if input.action_activated(InputAction::Spawn) || !self.spawned_startup_box {
            let player_transform = self.world.query_one_mut::<&Transform>(self.player).unwrap();
            let pos = if self.spawned_startup_box {
                player_transform.position() + player_transform.forward().xyz() * 5.0
            } else {
                self.spawned_startup_box = true;
                Vec3::y_axis().xyz() * 5.0
            };
            self.spawn_box(pos, Vec3::from_element(1.0), gfx, assets);
        }

        self.sync_physics();

        if let Some(new_size) = new_canvas_size {
            self.handle_canvas_resize(new_size, gfx, assets);
        }
    }

    pub fn render(&mut self, gfx: &Graphics, assets: &mut Assets) {
        self.render_with_camera(self.player, gfx, assets);
        self.render_with_camera(self.postprocessor, gfx, assets);
    }

    fn handle_canvas_resize(
        &mut self,
        new_size: &SurfaceSize,
        gfx: &Graphics,
        assets: &mut Assets,
    ) {
        let mut player_cam = self.world.get::<&mut Camera>(self.player).unwrap();
        player_cam.set_aspect(new_size.width as f32 / new_size.height as f32);
        player_cam
            .target_mut()
            .unwrap()
            .resize((new_size.width, new_size.height), gfx);

        let color_tex = player_cam.target().as_ref().unwrap().color_tex();
        let mut material = self.world.get::<&mut Material>(self.postprocessor).unwrap();
        assets.remove_material(material.0);
        material.0 = assets.add_postprocess_material(gfx, color_tex);
    }

    fn spawn_floor(&mut self, gfx: &Graphics, assets: &mut Assets) {
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
        let material = assets.add_textured_material(gfx, assets.bricks_texture);
        self.world.spawn((
            Transform::new(pos, scale),
            Mesh(assets.box_mesh),
            Material(material),
            body,
            RenderOrder(0),
            RenderTags(RENDER_TAG_SCENE),
        ));
    }

    fn spawn_box(&mut self, pos: Vec3, scale: Vec3, gfx: &Graphics, assets: &mut Assets) {
        let body = RigidBody::cuboid(
            RigidBodyParams {
                pos,
                scale,
                movable: true,
            },
            &mut self.physics,
        );
        let material = assets.add_textured_material(gfx, assets.crate_texture);
        self.world.spawn((
            Transform::new(pos, scale),
            Mesh(assets.box_mesh),
            Material(material),
            body,
            RenderOrder(0),
            RenderTags(RENDER_TAG_SCENE),
        ));
    }

    fn render_with_camera(&mut self, camera: Entity, gfx: &Graphics, assets: &mut Assets) {
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
                        materials::Material::Color(m) => m.set_wvp(gfx, cam, cam_tr, transform),
                        materials::Material::Skybox(m) => m.set_wvp(gfx, cam, cam_tr),
                        materials::Material::Textured(m) => m.set_wvp(gfx, cam, cam_tr, transform),
                        materials::Material::PostProcess(_) => (),
                    }
                    gfx.build_render_bundle(mesh.0, material.0, cam.target().as_ref(), assets)
                })
                // TODO Avoid vec allocation
                .collect::<Vec<wgpu::RenderBundle>>();

            gfx.render_pass(&bundles, cam.target().as_ref());
        }
    }

    fn sync_physics(&mut self) {
        for (_, (t, b)) in self.world.query_mut::<(&mut Transform, &RigidBody)>() {
            let body = self.physics.bodies.get(b.handle()).unwrap();
            t.set(*body.translation(), *body.rotation().inverse().quaternion());
        }
    }
}
