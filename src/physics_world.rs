use crate::math::{from_na_point, from_na_vec3, to_na_point, to_na_vec3};
use cgmath::Vector3;
use rapier3d::control::{EffectiveCharacterMovement, KinematicCharacterController};
use rapier3d::prelude::*;

pub struct PhysicsWorld {
    pub bodies: RigidBodySet,
    pub colliders: ColliderSet,
    pub query_pipeline: QueryPipeline,
    physics_pipeline: PhysicsPipeline,
    island_manager: IslandManager,
    broad_phase: BroadPhase,
    narrow_phase: NarrowPhase,
    impulse_joints: ImpulseJointSet,
    multibody_joints: MultibodyJointSet,
    ccd_solver: CCDSolver,
    char_controller: KinematicCharacterController,
}

impl PhysicsWorld {
    pub fn new() -> Self {
        let bodies = RigidBodySet::new();
        let colliders = ColliderSet::new();
        let physics_pipeline = PhysicsPipeline::new();
        let query_pipeline = QueryPipeline::new();
        let island_manager = IslandManager::new();
        let broad_phase = BroadPhase::new();
        let narrow_phase = NarrowPhase::new();
        let impulse_joints = ImpulseJointSet::new();
        let multibody_joints = MultibodyJointSet::new();
        let ccd_solver = CCDSolver::new();
        let char_controller = KinematicCharacterController::default();

        Self {
            bodies,
            colliders,
            physics_pipeline,
            query_pipeline,
            island_manager,
            broad_phase,
            narrow_phase,
            impulse_joints,
            multibody_joints,
            ccd_solver,
            char_controller,
        }
    }

    pub fn add_body(
        &mut self,
        body: RigidBody,
        collider: Collider,
    ) -> (RigidBodyHandle, ColliderHandle) {
        let body_handle = self.bodies.insert(body);
        let collider_handle =
            self.colliders
                .insert_with_parent(collider, body_handle, &mut self.bodies);
        (body_handle, collider_handle)
    }

    pub fn move_character(
        &self,
        dt: f32,
        desired_translation: Vector3<f32>,
        collider_handle: ColliderHandle,
    ) -> (Vector3<f32>, Vector3<f32>) {
        let (EffectiveCharacterMovement { translation, .. }, collider_current_pos) = {
            let (collider_pos, collider_shape) = {
                let collider = self.colliders.get(collider_handle).unwrap();
                (collider.position(), collider.shape())
            };

            let effective_movement = self.char_controller.move_shape(
                dt,
                &self.bodies,
                &self.colliders,
                &self.query_pipeline,
                collider_shape,
                collider_pos,
                to_na_vec3(desired_translation),
                QueryFilter::default().exclude_collider(collider_handle),
                |_| {},
            );

            (effective_movement, collider_pos.translation.vector)
        };

        (
            from_na_vec3(translation),
            from_na_vec3(collider_current_pos),
        )
    }

    pub fn cast_ray(
        &self,
        from: Vector3<f32>,
        dir: Vector3<f32>,
        exclude: Option<ColliderHandle>,
    ) -> Option<(Vector3<f32>, Vector3<f32>, ColliderHandle)> {
        let ray = Ray {
            origin: to_na_point(from),
            // Inverting the dir because I don't know why :(
            // Maybe Rapier uses a different basis orientation or smth.
            dir: to_na_vec3(-dir),
        };

        let mut filter = QueryFilter::default();
        if let Some(exclude_collider_handle) = exclude {
            filter = filter.exclude_collider(exclude_collider_handle);
        }

        if let Some((handle, intersection)) = self.query_pipeline.cast_ray_and_get_normal(
            &self.bodies,
            &self.colliders,
            &ray,
            Real::MAX,
            true,
            filter,
        ) {
            let hit_pt = ray.point_at(intersection.toi);
            return Some((from_na_point(hit_pt), from_na_vec3(intersection.normal), handle));
        }

        None
    }

    pub fn update(&mut self, dt: f32) {
        let gravity = vector![0.0, -9.81, 0.0];
        let integration_parameters = IntegrationParameters {
            dt,
            ..IntegrationParameters::default()
        };

        self.physics_pipeline.step(
            &gravity,
            &integration_parameters,
            &mut self.island_manager,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.bodies,
            &mut self.colliders,
            &mut self.impulse_joints,
            &mut self.multibody_joints,
            &mut self.ccd_solver,
            None,
            &(),
            &(),
        );

        self.query_pipeline.update(&self.bodies, &self.colliders);
    }
}
