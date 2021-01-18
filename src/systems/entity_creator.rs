use std::cell::RefCell;
use std::rc::Rc;

use nalgebra::{Isometry3, Vector3};
use ncollide3d::shape::ShapeHandle;
use ncollide3d::transformation::ToTriMesh;
use nphysics3d::material::{BasicMaterial, MaterialHandle};
use nphysics3d::object::{BodyPartHandle, BodyStatus, ColliderDesc, Ground, RigidBodyDesc};
use specs::{Entities, System, Write, WriteExpect, WriteStorage};

use crate::{components::*, entities::EntityType, resources, resources::EntityQueue};
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

                    let translation = Vector3::from(point.coords);
                    let rotation = Vector3::new(0.0, 0.0, 0.0);
                    let transform = Isometry3::new(translation, rotation);

                    // node.set_local_transformation(transform.clone());

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

                    let shape = ShapeHandle::new(ncollide3d::shape::Ball::new(radius));
                    let collider = ColliderDesc::new(shape)
                        .material(MaterialHandle::new(BasicMaterial::new(2.0, 0.8)))
                        .margin(0.5)
                        .linear_prediction(0.25)
                        .build(BodyPartHandle(body_handle, 0));
                    let collider_handle = physics_world.colliders.insert(collider);

                    let gfx_id = game_context.store_gfx(node);
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
                EntityType::Ground { thickness } => {
                    let ground_shape =
                        ncollide3d::shape::Cuboid::new(Vector3::new(30.0, thickness, 30.0));
                    let mut ground_node = game_context
                        .window_mut()
                        .add_trimesh(ground_shape.to_trimesh(()), Vector3::new(1.0, 1.0, 1.0));

                    let translation = Vector3::new(0.0, -thickness, 0.0);
                    let rotation = Vector3::new(0.0, 0.0, 0.0);

                    ground_node
                        .set_local_transformation(Isometry3::new(translation.clone(), rotation));
                    ground_node.set_color(0.0, 0.5, 0.25);

                    let ground_handle = physics_world.bodies.insert(Ground::new());

                    let co = ColliderDesc::new(ShapeHandle::new(ground_shape))
                        .translation(translation)
                        .build(BodyPartHandle(ground_handle, 0));
                    physics_world.colliders.insert(co);
                }
            }
        }
    }
}
