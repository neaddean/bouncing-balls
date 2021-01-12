use kiss3d::event::{Key, Modifiers};
use nalgebra::Point3;
use rand::{Rng, thread_rng};
use specs::{join::Join, ReadStorage, System, Write, WriteExpect, WriteStorage};

use crate::{
    components::*,
    resources::{EntityQueue, EventQueue},
};
use crate::entities::EntityType;
use crate::resources::GameState;
use crate::systems::event_types::Event;

pub struct EventSystem;

// System implementation
impl<'a> System<'a> for EventSystem {
    // Data
    type SystemData = (
        Write<'a, EventQueue>,
        Write<'a, EntityQueue>,
        WriteStorage<'a, Velocity>,
        ReadStorage<'a, Ball>,
        WriteExpect<'a, GameState>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut event_queue,
            mut entity_queue,
            mut velocities,
            balls,
            mut game_state,
        ) = data;

        for event in event_queue.events.drain(..) {
            // println!("New event: {:?}", event);
            match event {
                Event::KeyDown(key_code, _key_mods, _is_repeated) => {
                    match (key_code, _is_repeated, _key_mods) {
                        (Key::Tab, false, Modifiers::Shift) => {
                            // game_state.show_debug ^= true;
                        }
                        // (Key::Up, ..) => {
                        //     for (vel, _) in (&mut velocities, &balls).join() {
                        //         vel.x += 120.0 * num::signum(vel.x);
                        //         vel.y += 120.0 * num::signum(vel.y);
                        //     }
                        // }
                        // (Key::Down, ..) => {
                        //     for (vel, _) in (&mut velocities, &balls).join() {
                        //         vel.x -= 120.0 * num::signum(vel.x);
                        //         vel.y -= 120.0 * num::signum(vel.y);
                        //     }
                        // }
                        (Key::Space, ..) => {
                            entity_queue.push(EntityType::Ball {
                                point: Point3::new(
                                    thread_rng().gen_range(-120.0..120.0),
                                    thread_rng().gen_range(-120.0..120.0),
                                    thread_rng().gen_range(-120.0..120.0),
                                ),
                                radius: 25.0,
                            });
                        }
                        (Key::F, false, Modifiers::Control) => {
                            game_state.show_fps ^= true;
                        }
                        (Key::L, false, Modifiers::Control) => {
                            game_state.sw_frame_limiter ^= true;
                        }
                        (Key::Escape, false, _) => {
                            game_state.continuing = false;
                        }
                        _ => {}
                    }
                }
                Event::CloseGame => {
                    game_state.continuing = false;
                }
                Event::WindowSize(_, _) => {}
                Event::KeyUp(_, _) => {}
            }
        }
    }
}
