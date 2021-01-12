use std::cell::RefCell;
use std::rc::Rc;

use kiss3d::window::{CanvasSetup, NumSamples, Window};
use nalgebra::Point3;
use specs::{DispatcherBuilder, World, WorldExt};

use balz::context::GameContext;
use balz::entities;
use balz::resources::{EntityQueue, GameState};
use balz::systems::*;

fn main() {
    let canvas_config = CanvasSetup { vsync: false, samples: NumSamples::Two };
    let window = Window::new_with_setup("asdf", 800, 600, canvas_config);

    let game_context = Rc::new(RefCell::new(GameContext::new(window)));

    let ref mut world = World::new();
    world.insert(GameState::new(&mut game_context.borrow_mut()));

    let ref mut dispatcher = DispatcherBuilder::new()
        .with(EventSystem, "events", &[])
        .with_thread_local(EntityCreatorSystem::new(Rc::clone(&game_context)))
        .with_thread_local(PhysicsSystem::new(Rc::clone(&game_context)))
        .with_thread_local(EntityRemovalSystem::new(Rc::clone(&game_context)))
        .with_thread_local(InputSystem::new(Rc::clone(&game_context)))
        .with_thread_local(UpdateRenderablesSystem::new(Rc::clone(&game_context)))
        .with_thread_local(RenderingSystem::new(Rc::clone(&game_context)))
        .build();

    dispatcher.setup(world);

    {
        let mut entity_queue = world.write_resource::<EntityQueue>();
        entity_queue.push(entities::EntityType::Ball {
            point: Point3::new(0.0, 0.0, 100.0),
            radius: 25.0,
        });
    }

    balz::gameloop::run(dispatcher, world);
}

