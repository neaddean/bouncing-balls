use std::cell::RefCell;
use std::rc::Rc;

use log::trace;
use specs::{join::Join, ReadExpect, ReadStorage, System};

use crate::components::*;
use crate::context::GameContext;
use crate::resources;

pub struct UpdateRenderablesSystem {
    game_context: Rc<RefCell<GameContext>>,
}

impl UpdateRenderablesSystem {
    pub fn new(game_context: Rc<RefCell<GameContext>>) -> Self {
        UpdateRenderablesSystem { game_context }
    }
}

impl<'a> System<'a> for UpdateRenderablesSystem {
    type SystemData = (ReadStorage<'a, Physical>,
                       ReadStorage<'a, Renderable>,
                       ReadExpect<'a, resources::PhysicsWorld>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (physicals,
            renderables,
            physical_world,
        ) = data;
        let ref mut game_context = self.game_context.borrow_mut();

        for (phys, rend) in (&physicals, &renderables).join() {
            let col = physical_world.colliders.get(phys.collider_handle).unwrap();
            let mut gfx_node = game_context.get_gfx(rend.gfx_id);
            let pos = *col.position();
            trace!("{:?}", &pos);
            gfx_node.set_local_transformation(pos);
        }
    }
}
