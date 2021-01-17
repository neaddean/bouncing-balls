use std::cell::RefCell;
use std::rc::Rc;

use specs::{Entities, join::Join, ReadExpect, ReadStorage, System, Write, WriteStorage, WriteExpect};

use crate::components::*;
use crate::constants::SIMULATION_DURATION;
use crate::context::GameContext;
use crate::resources;
use crate::resources::EntityRemovalQueue;

pub struct PhysicsSystem {
    game_context: Rc<RefCell<GameContext>>,
    accum: f32,
}

impl PhysicsSystem {
    pub fn new(game_context: Rc<RefCell<GameContext>>) -> Self {
        PhysicsSystem {
            game_context,
            accum: 0.0,
        }
    }
}

impl<'a> System<'a> for PhysicsSystem {
    type SystemData = (
        WriteStorage<'a, Position>,
        ReadStorage<'a, Ball>,
        ReadExpect<'a, resources::GameState>,
        Write<'a, EntityRemovalQueue>,
        WriteExpect<'a, resources::PhysicsWorld>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut positions,
            balls,
            game_state,
            _entity_removal_queue,
            _,
            entities) =
            data;

        self.accum += game_state.this_duration().as_secs_f32();
        while self.accum > SIMULATION_DURATION {
            self.accum -= SIMULATION_DURATION;

            for (_entity, _position, _ball) in
            (&entities, &mut positions, &balls).join()
            {}
        }
    }
}
