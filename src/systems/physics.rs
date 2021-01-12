use std::cell::RefCell;
use std::rc::Rc;

use specs::{Entities, join::Join, ReadExpect, ReadStorage, System, Write, WriteStorage};

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
    pub fn new(
        game_context: Rc<RefCell<GameContext>>,
    ) -> Self {
        PhysicsSystem {
            game_context,
            accum: 0.0,
        }
    }
}

impl<'a> System<'a> for PhysicsSystem {
    type SystemData = (
        WriteStorage<'a, Position>,
        WriteStorage<'a, Velocity>,
        ReadStorage<'a, Ball>,
        ReadExpect<'a, resources::GameState>,
        Write<'a, EntityRemovalQueue>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut positions,
            mut velocities,
            balls,
            game_state,
            mut entity_removal_queue,
            entities) = data;

        self.accum += game_state.this_duration().as_secs_f32();
        while self.accum > SIMULATION_DURATION {
            self.accum -= SIMULATION_DURATION;

            for (entity, position, velocity, ball) in
            (&entities, &mut positions, &mut velocities, &balls).join()
            {
            }
        }
    }
}