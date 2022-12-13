use nalgebra::{RealField, Vector3};
use nphysics3d::force_generator::DefaultForceGeneratorSet;
use nphysics3d::joint::DefaultJointConstraintSet;
use nphysics3d::object::{DefaultBodySet, DefaultColliderSet};
use nphysics3d::solver::SignoriniModel;
use nphysics3d::world::{DefaultGeometricalWorld, DefaultMechanicalWorld};
use tracing::{debug};

use crate::constants::SIMULATION_DURATION;

pub struct PhysicsWorldN<N: RealField> {
    pub mechanical_world: DefaultMechanicalWorld<N>,
    pub geometrical_world: DefaultGeometricalWorld<N>,
    pub bodies: DefaultBodySet<N>,
    pub colliders: DefaultColliderSet<N>,
    pub joint_constraints: DefaultJointConstraintSet<N>,
    pub force_generators: DefaultForceGeneratorSet<N>,
}

pub type PhysicsWorld = PhysicsWorldN<f32>;

impl PhysicsWorld {
    pub fn new() -> Self {
        let mut mechanical_world = DefaultMechanicalWorld::new(Vector3::new(0.0, -9.81, 0.0));
        mechanical_world.set_timestep(SIMULATION_DURATION);
        mechanical_world.solver.set_contact_model(Box::new(SignoriniModel::new()));
        let geometrical_world = DefaultGeometricalWorld::new();
        let bodies = DefaultBodySet::new();
        let colliders = DefaultColliderSet::<f32>::new();
        let joint_constraints = DefaultJointConstraintSet::new();
        let force_generators = DefaultForceGeneratorSet::new();
        PhysicsWorld {
            mechanical_world,
            geometrical_world,
            bodies,
            colliders,
            joint_constraints,
            force_generators,
        }
    }

    // #[instrument(skip(self))]
    pub fn step(&mut self) {
        debug!("stepping physics");
        self.mechanical_world.step(
            &mut self.geometrical_world,
            &mut self.bodies,
            &mut self.colliders,
            &mut self.joint_constraints,
            &mut self.force_generators,
        );
    }

    pub fn maintain(&mut self) {
        self.mechanical_world.maintain(
            &mut self.geometrical_world,
            &mut self.bodies,
            &mut self.colliders,
            &mut self.joint_constraints,
        );
    }
}
