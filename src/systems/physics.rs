use std::cell::RefCell;
use std::rc::Rc;

use specs::{Entities, join::Join, Read, ReadExpect, ReadStorage, System, Write, WriteExpect, WriteStorage};

use crate::components::*;
use crate::constants::SIMULATION_DURATION;
use crate::context::GameContext;
use crate::resources;
use crate::resources::{EntityRemovalQueue, GameState};

pub struct PhysicsSystem {
    accum: f32,
}

impl PhysicsSystem {
    pub fn new() -> Self {
        PhysicsSystem {
            accum: 0.0,
        }
    }
}

impl<'a> System<'a> for PhysicsSystem {
    type SystemData = (
        ReadExpect<'a, GameState>,
        WriteExpect<'a, resources::PhysicsWorld>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            game_state,
            mut physical_world) =
            data;

        self.accum += game_state.this_duration().as_secs_f32();
        while self.accum > SIMULATION_DURATION {
            self.accum -= SIMULATION_DURATION;
            physical_world.step();
        }
    }
}
