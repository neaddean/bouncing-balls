use std::time::Instant;

use specs::{ReadExpect, System, WriteExpect};
use tracing::debug;

use crate::constants::SIMULATION_DURATION;
use crate::resources;
use crate::resources::GameState;

pub struct PhysicsSystem {
    accum: f32,
}

impl PhysicsSystem {
    pub fn new() -> Self {
        PhysicsSystem { accum: 0.0 }
    }
}

impl<'a> System<'a> for PhysicsSystem {
    type SystemData = (
        ReadExpect<'a, GameState>,
        WriteExpect<'a, resources::PhysicsWorld>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (game_state, mut physical_world) = data;

        self.accum += game_state.this_duration().as_secs_f32();
        let mut accum2: u32 = 0;
        while self.accum > SIMULATION_DURATION {
            let now = Instant::now();
            self.accum -= SIMULATION_DURATION;
            physical_world.step();
            debug!("physics step: {}", now.elapsed().as_micros());
            accum2 += 1;
        }
        debug!("ran {} loops", accum2);
    }
}
