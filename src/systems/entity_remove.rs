use std::cell::RefCell;
use std::rc::Rc;

use specs::{Entities, ReadStorage, System, Write, WriteExpect};

use crate::components::*;
use crate::context::GameContext;
use crate::resources::{EntityRemovalQueue, PhysicsWorld};

pub struct EntityRemovalSystem {
    game_context: Rc<RefCell<GameContext>>,
}

impl EntityRemovalSystem {
    pub fn new(game_context: Rc<RefCell<GameContext>>) -> Self {
        EntityRemovalSystem { game_context }
    }
}

// System implementation
impl<'a> System<'a> for EntityRemovalSystem {
    // Data
    type SystemData = (
        Write<'a, EntityRemovalQueue>,
        Entities<'a>,
        ReadStorage<'a, Renderable>,
        ReadStorage<'a, Physical>,
        WriteExpect<'a, PhysicsWorld>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut entity_removal_queue,
            entites,
            renderables,
            physicals,
            mut physical_world,
        ) = data;

        let ref mut game_context = self.game_context.borrow_mut();
        for entity_to_delete in entity_removal_queue.drain(..) {
            entites.delete(entity_to_delete).unwrap();
            let this_renderable = renderables.get(entity_to_delete).unwrap();
            game_context.remove_gfx(this_renderable.gfx_id);

            let this_physical = physicals.get(entity_to_delete).unwrap();
            physical_world.bodies.remove(this_physical.body_handle);
            physical_world.colliders.remove(this_physical.collider_handle);
        }
        // physical_world.maintain();
    }
}
