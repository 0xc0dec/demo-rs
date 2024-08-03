use rapier3d::control::{EffectiveCharacterMovement, KinematicCharacterController};
use rapier3d::prelude::*;

use crate::math::Vec3;

// TODO Try to fully encapsulate Rapier types
pub struct Physics {
    pub bodies: RigidBodySet,
    pub colliders: ColliderSet,
    query_pipeline: QueryPipeline,
    physics_pipeline: PhysicsPipeline,
    island_manager: IslandManager,
    broad_phase: Box<dyn BroadPhase>,
    narrow_phase: NarrowPhase,
    impulse_joints: ImpulseJointSet,
    multibody_joints: MultibodyJointSet,
    ccd_solver: CCDSolver,
    char_controller: KinematicCharacterController,
}

pub struct RayCastResult {
    // Distance of the hit along the ray
    pub distance: f32,
    // Body that was hit
    pub collider: ColliderHandle,
}

impl Physics {
    pub fn new() -> Self {
        Self {
            bodies: RigidBodySet::new(),
            colliders: ColliderSet::new(),
            physics_pipeline: PhysicsPipeline::new(),
            query_pipeline: QueryPipeline::new(),
            island_manager: IslandManager::new(),
            broad_phase: Box::new(BroadPhaseMultiSap::new()),
            narrow_phase: NarrowPhase::new(),
            impulse_joints: ImpulseJointSet::new(),
            multibody_joints: MultibodyJointSet::new(),
            ccd_solver: CCDSolver::new(),
            char_controller: KinematicCharacterController::default(),
        }
    }

    pub fn add_body(&mut self, body: RigidBody, collider: Collider) -> RigidBodyHandle {
        let body = self.bodies.insert(body);
        self.colliders
            .insert_with_parent(collider, body, &mut self.bodies);
        body
    }

    pub fn move_character(
        &self,
        dt: f32,
        desired_translation: Vec3,
        collider: ColliderHandle,
    ) -> (Vec3, Vec3) {
        let (EffectiveCharacterMovement { translation, .. }, collider_current_pos) = {
            let (collider_pos, collider_shape) = {
                let collider = self.colliders.get(collider).unwrap();
                (collider.position(), collider.shape())
            };

            let possible_movement = self.char_controller.move_shape(
                dt,
                &self.bodies,
                &self.colliders,
                &self.query_pipeline,
                collider_shape,
                collider_pos,
                desired_translation,
                QueryFilter::default().exclude_collider(collider),
                |_| {},
            );

            (possible_movement, collider_pos.translation.vector)
        };

        (translation, collider_current_pos)
    }

    pub fn cast_ray(
        &self,
        from: Vec3,
        dir: Vec3,
        exclude: Option<ColliderHandle>,
    ) -> Option<RayCastResult> {
        let ray = Ray {
            origin: from.into(),
            dir,
        };

        let mut filter = QueryFilter::default();
        if let Some(exclude_collider) = exclude {
            filter = filter.exclude_collider(exclude_collider);
        }

        if let Some((handle, intersection)) = self.query_pipeline.cast_ray_and_get_normal(
            &self.bodies,
            &self.colliders,
            &ray,
            Real::MAX,
            true,
            filter,
        ) {
            return Some(RayCastResult {
                distance: intersection.time_of_impact,
                collider: handle,
            });
        }

        None
    }

    pub fn update(&mut self, dt: f32) {
        let gravity = vector![0.0, -9.81, 0.0];
        let params = IntegrationParameters {
            dt,
            ..IntegrationParameters::default()
        };

        self.physics_pipeline.step(
            &gravity,
            &params,
            &mut self.island_manager,
            self.broad_phase.as_mut(),
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

        self.query_pipeline.update(&self.colliders);
    }
}
