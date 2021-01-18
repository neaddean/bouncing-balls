use std::cell::RefCell;
use std::rc::Rc;

use kiss3d::window::{CanvasSetup, NumSamples, Window};
use nalgebra as na;
use specs::{DispatcherBuilder, World, WorldExt};

// use kiss3d::camera::ArcBall;
use balz::camera::ArcBall;
use balz::context::GameContext;
use balz::entities;
use balz::resources::{CameraBox, EntityQueue, GameState, PhysicsWorld};
use balz::systems::*;

fn main() {
    balz::setup_logging();

    let canvas_config = CanvasSetup {
        vsync: false,
        samples: NumSamples::Two,
    };
    let window = Window::new_with_setup("asdf", 800, 600, canvas_config);

    let game_context = Rc::new(RefCell::new(GameContext::new(window)));

    let ref mut world = World::new();
    world.insert(GameState::new(&mut game_context.borrow_mut()));

    world.insert(PhysicsWorld::new());

    {
        let eye = na::Point3::new(10.0, 30.0, 10.0);
        let at = na::Point3::origin();
        let mut camera = ArcBall::new_with_frustrum(std::f32::consts::PI / 4.0, 0.1, 1024.0, eye, at);
        camera.set_max_pitch(std::f32::consts::PI / 2.0 * 0.92);
        camera.set_min_dist(5.0);
        camera.set_max_dist(100.0);
        world.insert(CameraBox { camera: Box::new(camera) });
    }
    let ref mut dispatcher = DispatcherBuilder::new()
        .with(EventSystem, "events", &[])
        // .with(PhysicsSystem::new(), "physics", &["events"])
        .with_thread_local(EntityCreatorSystem::new(Rc::clone(&game_context)))
        .with_thread_local(PhysicsSystem::new())
        .with_thread_local(InputSystem::new(Rc::clone(&game_context)))
                          .with_thread_local(UpdateRenderablesSystem::new(Rc::clone(&game_context)))
        .with_thread_local(RenderingSystem::new(Rc::clone(&game_context)))
        .build();

    dispatcher.setup(world);

    {
        let mut entity_queue = world.write_resource::<EntityQueue>();
        entity_queue.push(entities::EntityType::Ball {
            point: na::Point3::new(0.0, 10.0, 0.0),
            radius: 0.25,
        });
        entity_queue.push(entities::EntityType::Ground { thickness: 0.1 });
    }

    balz::gameloop::run(dispatcher, world);
}
