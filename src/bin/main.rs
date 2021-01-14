use std::cell::RefCell;
use std::rc::Rc;

use kiss3d::camera::{ArcBall, FirstPerson};
use kiss3d::window::{CanvasSetup, NumSamples, Window};
use nalgebra::{Point3, Translation3};
use specs::{DispatcherBuilder, World, WorldExt};

use balz::context::GameContext;
use balz::entities;
use balz::resources::{CameraBox, EntityQueue, GameState};
use balz::systems::*;

fn main() {
    let canvas_config = CanvasSetup { vsync: true, samples: NumSamples::Two };
    let window = Window::new_with_setup("asdf", 800, 600, canvas_config);

    let game_context = Rc::new(RefCell::new(GameContext::new(window)));

    let ref mut world = World::new();
    world.insert(GameState::new(&mut game_context.borrow_mut()));

    let eye = Point3::new(0.0, -30.0, 40.0);
    let at = Point3::origin();
    world.insert(CameraBox { camera: Box::new(ArcBall::new(eye, at)) });

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
            point: Point3::new(0.0, 0.0, 1.0),
            radius: 0.25,
        });
    }
    {
        let mut game_context = game_context.borrow_mut();
        let window = game_context.window_mut();
        let mut floor = window.add_quad(15.0, 15.0, 100, 100);
        floor.set_local_translation(Translation3::new(0.0, 0.0, 0.0));
        floor.set_color(0.0, 0.5, 0.25);
        game_context.store_gfx(floor);
    }

    balz::gameloop::run(dispatcher, world);
}

