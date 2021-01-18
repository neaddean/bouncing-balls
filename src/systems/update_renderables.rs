use std::cell::RefCell;
use std::rc::Rc;

use specs::{Entities, join::Join, ReadStorage, System, Write, WriteExpect};
use tracing::trace;

use crate::components::*;
use crate::context::GameContext;
use crate::resources;
use crate::resources::EntityRemovalQueue;

pub struct UpdateRenderablesSystem {
    game_context: Rc<RefCell<GameContext>>,
}

impl UpdateRenderablesSystem {
    pub fn new(game_context: Rc<RefCell<GameContext>>) -> Self {
        UpdateRenderablesSystem { game_context }
    }
}

impl<'a> System<'a> for UpdateRenderablesSystem {
    type SystemData = (
        ReadStorage<'a, Physical>,
        ReadStorage<'a, Renderable>,
        WriteExpect<'a, resources::PhysicsWorld>,
        Write<'a, EntityRemovalQueue>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (physicals,
            renderables,
            mut physical_world,
            mut entity_removal_queue,
            entities,
        ) = data;
        let ref mut game_context = self.game_context.borrow_mut();
        physical_world.maintain();
        for (phys, rend, entity) in (&physicals, &renderables, &entities).join() {
            let col = physical_world.colliders.get(phys.collider_handle);
            // need to fix this. world and physics maintain must be called together bfore these steps
            if col.is_none(){
                continue
            }
            let col = col.unwrap();
            let mut gfx_node = game_context.get_gfx(rend.gfx_id);
            let pos = *col.position();
            trace!("{:?}", &pos);

            if pos.translation.y < -1.0 {
                entity_removal_queue.push(entity);
            }
            gfx_node.set_local_transformation(pos);
        }
    }
}
