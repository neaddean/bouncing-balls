use std::cell::RefCell;
use std::rc::Rc;

use nalgebra::{Isometry3, Vector3};
use nphysics3d::object::{BodyStatus, ColliderDesc, RigidBodyDesc, BodyPartHandle};
use specs::{Entities, System, Write, WriteExpect, WriteStorage};

use crate::{components::*, entities::EntityType, resources::{EntityQueue}, resources};
use crate::context::GameContext;

pub struct EntityCreatorSystem {
    game_context: Rc<RefCell<GameContext>>,
}

impl EntityCreatorSystem {
    pub fn new(game_context: Rc<RefCell<GameContext>>) -> Self {
        EntityCreatorSystem { game_context }
    }
}

// System implementation
impl<'a> System<'a> for EntityCreatorSystem {
    // Data
    type SystemData = (
        Write<'a, EntityQueue>,
        Entities<'a>,
        WriteStorage<'a, Physical>,
        WriteStorage<'a, Renderable>,
        WriteStorage<'a, Ball>,
        WriteExpect<'a, resources::PhysicsWorld>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut entity_queue,
            entites,
            mut physicals,
            mut renderables,
            mut ball_storage,
            mut physics_world,
        ) = data;

        let ref mut game_context = self.game_context.borrow_mut();
        // let mut physics_world = physics_world.borrow_mut();
        for entity_to_create in entity_queue.drain(..) {
            match entity_to_create {
                EntityType::Ball { point, radius } => {
                    // create node
                    let mut node = game_context.window_mut().add_sphere(radius);
                    node.set_color(1.0, 0.0, 0.0);
                    let gfx_id = game_context.store_gfx(node);

                    let translation = Vector3::from(point.coords);
                    let rotation = Vector3::new(0.0, 0.0, 0.0);
                    let transform = Isometry3::new(translation, rotation);

                    let rigid_body = RigidBodyDesc::new()
                        .position(transform.clone())
                        .gravity_enabled(true)
                        .status(BodyStatus::Dynamic)
                        // .linear_damping(10.0)
                        .mass(1.0)
                        .kinematic_rotations(Vector3::new(false, false, false))
                        .linear_motion_interpolation_enabled(true)
                        .build();
                    let body_handle = physics_world.bodies.insert(rigid_body);

                    let shape = ncollide3d::shape::ShapeHandle::new(ncollide3d::shape::Ball::new(1.5));
                    let collider = ColliderDesc::new(shape)
                        .build(BodyPartHandle(body_handle, 0));
                    let collider_handle = physics_world.colliders.insert(collider);

                    entites
                        .build_entity()
                        .with(Ball { radius }, &mut ball_storage)
                        .with(
                            Physical {
                                body_handle,
                                collider_handle,
                            },
                            &mut physicals,
                        )
                        .with(Renderable { gfx_id }, &mut renderables)
                        .build();
                }
            }
        }
    }
}
