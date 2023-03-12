use cgmath::Vector3;
use rapier3d::control::{EffectiveCharacterMovement, KinematicCharacterController};
use rapier3d::prelude::*;
use crate::math::to_na_vec3;

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
            char_controller
        }
    }

    pub fn add_body(&mut self, body: RigidBody, collider: Collider) -> (RigidBodyHandle, ColliderHandle) {
        let body_handle = self.bodies.insert(body);
        let collider_handle = self.colliders.insert_with_parent(collider, body_handle, &mut self.bodies);
        (body_handle, collider_handle)
    }

    pub fn move_character(
        &self,
        dt: f32,
        desired_translation: Vector3<f32>,
        collider_handle: ColliderHandle
    ) -> (Vector3<f32>, Vector3<f32>) {
        let (EffectiveCharacterMovement { translation, .. }, collider_current_pos) = {
            let (collider_pos, collider_shape) = {
                let collider = self.colliders
                    .get(collider_handle)
                    .unwrap();
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
                QueryFilter::default()
                    .exclude_collider(collider_handle),
                |_| {}
            );

            (effective_movement, collider_pos.translation.vector)
        };

        (
            Vector3::new(translation.x, translation.y, translation.z),
            Vector3::new(collider_current_pos.x, collider_current_pos.y, collider_current_pos.z)
        )
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