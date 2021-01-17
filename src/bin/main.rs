use std::cell::RefCell;
use std::rc::Rc;

use kiss3d::camera::ArcBall;
use kiss3d::window::{CanvasSetup, NumSamples, Window};
use nalgebra as na;
use specs::{DispatcherBuilder, World, WorldExt};

use balz::context::GameContext;
use balz::entities;
use balz::resources::{CameraBox, EntityQueue, GameState, PhysicsWorld};
use balz::systems::*;

fn main() {
    simplelog::SimpleLogger::init(
        simplelog::LevelFilter::Debug,
        simplelog::ConfigBuilder::new()
            // .add_filter_allow_str("balz")
            .set_time_format("%H:%M:%S%.3f".to_string())
            .build(),
    );

    let canvas_config = CanvasSetup {
        vsync: false,
        samples: NumSamples::Two,
    };
    let window = Window::new_with_setup("asdf", 800, 600, canvas_config);

    let game_context = Rc::new(RefCell::new(GameContext::new(window)));

    let ref mut world = World::new();
    world.insert(GameState::new(&mut game_context.borrow_mut()));

    world.insert(PhysicsWorld::new());

    let eye = na::Point3::new(10.0, 30.0, 10.0);
    let at = na::Point3::origin();
    world.insert(CameraBox {
        camera: Box::new(ArcBall::new_with_frustrum(std::f32::consts::PI / 4.0, 0.1, 8096.0, eye, at)),
    });

    let ref mut dispatcher = DispatcherBuilder::new()
        .with(EventSystem, "events", &[])
        .with(PhysicsSystem::new(), "physics", &["events"])
        .with_thread_local(EntityCreatorSystem::new(Rc::clone(&game_context)))
        .with_thread_local(EntityRemovalSystem::new(Rc::clone(&game_context)))
        .with_thread_local(InputSystem::new(Rc::clone(&game_context)))
        .with_thread_local(UpdateRenderablesSystem::new(Rc::clone(&game_context)))
        .with_thread_local(RenderingSystem::new(Rc::clone(&game_context)))
        .build();

    dispatcher.setup(world);

    {
        let mut entity_queue = world.write_resource::<EntityQueue>();
        entity_queue.push(entities::EntityType::Ball {
            point: na::Point3::new(0.0, 0.0, 1.0),
            radius: 0.25,
        });
    }
    {
        let mut game_context = game_context.borrow_mut();
        let window = game_context.window_mut();
        let mut floor = window.add_quad(15.0, 15.0, 100, 100);
        let translation = na::Vector3::new(0.0, 0.0, 0.0);
        let rotation = na::Vector3::new(std::f32::consts::PI / 2.0, 0.0, 0.0);
        floor.set_local_transformation(na::Isometry3::new(translation, rotation));
        floor.set_color(0.0, 0.5, 0.25);
        game_context.store_gfx(floor);
    }

    balz::gameloop::run(dispatcher, world);
}
