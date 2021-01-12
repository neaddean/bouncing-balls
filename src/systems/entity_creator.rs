use std::cell::RefCell;
use std::rc::Rc;

use nalgebra::{Isometry3, Vector3};
use specs::{Entities, ReadExpect, System, Write, WriteStorage};

use crate::{
    components::*,
    entities::EntityType,
    resources::{EntityQueue, GameState},
};
use crate::context::GameContext;

pub struct EntityCreatorSystem {
    game_context: Rc<RefCell<GameContext>>,
}


impl EntityCreatorSystem {
    pub fn new(
        game_context: Rc<RefCell<GameContext>>,
    ) -> Self {
        EntityCreatorSystem {
            game_context,
        }
    }
}

// System implementation
impl<'a> System<'a> for EntityCreatorSystem {
    // Data
    type SystemData = (
        Write<'a, EntityQueue>,
        Entities<'a>,
        WriteStorage<'a, Velocity>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, Renderable>,
        WriteStorage<'a, Ball>,
        ReadExpect<'a, GameState>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut entity_queue,
            entites,
            mut velocities,
            mut positions,
            mut renderables,
            mut ball_storage,
            game_state,
        ) = data;

        let ref mut game_context = self.game_context.borrow_mut();
        for entity_to_create in entity_queue.drain(..) {
            match entity_to_create {
                EntityType::Ball { point, radius } => {
                    let mut sphere = game_context.window_mut().add_sphere(radius);
                    sphere.set_color(1.0, 0.0, 0.0);

                    let translation = Vector3::from(point.coords);
                    let rotation = Vector3::new(0.0, 0.0, 0.0);
                    let transform = Isometry3::new(translation, rotation);
                    sphere.set_local_transformation(transform.clone());

                    let gfx_id = game_context.store_gfx(sphere);
                    entites
                        .build_entity()
                        .with(Ball {
                            radius
                        }, &mut ball_storage)
                        .with(Position {
                            position: transform,
                        }, &mut positions)
                        .with(Renderable { gfx_id }, &mut renderables)
                        .build();
                }
            }
        }
    }
}
