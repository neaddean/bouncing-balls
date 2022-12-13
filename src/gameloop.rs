use specs::{WorldExt, RunNow};

use crate::resources::GameState;

pub fn run(dispatcher: &mut specs::Dispatcher, world: &mut specs::World) {
    let mut caelex_system = crate::systems::caelex::CaelexSystem::new();
    loop {
        world.write_resource::<GameState>().tick();
        dispatcher.dispatch(world);
        world.maintain();
        caelex_system.run_now(world);
        std::thread::yield_now();
        if !world.write_resource::<GameState>().continuing {
            break;
        }
    }
}
