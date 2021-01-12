use std::cell::RefCell;
use std::rc::Rc;

use specs::{ReadStorage, System, WriteExpect};

use crate::components::*;
use crate::context::GameContext;
use crate::resources;

pub struct RenderingSystem {
    game_context: Rc<RefCell<GameContext>>,
    accum: f32,
}

impl RenderingSystem {
    pub fn new(
        game_context: Rc<RefCell<GameContext>>,
    ) -> Self {
        RenderingSystem {
            game_context,
            accum: 0.0,
        }
    }
}

impl<'a> System<'a> for RenderingSystem {
    type SystemData = (
        WriteExpect<'a, resources::GameState>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut game_state,
        ) = data;

        let ref mut game_context = self.game_context.borrow_mut();
        let ref mut window = game_context.window_mut();
        if game_state.sw_frame_limiter {
            self.accum += game_state.this_duration().as_secs_f32();
        } else {
            self.accum = 1.0 / game_state.sw_frame_limit_fps;
        }
        while self.accum >= 1.0 / game_state.sw_frame_limit_fps {
            self.accum -= 1.0 / game_state.sw_frame_limit_fps;
            game_state.continuing = window.render();
        }
    }
}
