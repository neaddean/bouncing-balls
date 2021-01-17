use kiss3d;
use specs;

pub use game_state::GameState;
pub use physics::PhysicsWorld;

mod game_state;
mod physics;

#[derive(Default)]
pub struct EventQueue {
    pub events: Vec<crate::systems::event_types::Event>,
}

// pub struct CameraBox {
//     camera: Box<dyn kiss3d::camera::Camera + Sync + Send>,
// }
//
// impl CameraBox {
//     pub fn new<T> (camera: dyn T) -> Self
//         where
//             T: kiss3d::camera::Camera + Sync + Send,
//     {
//         CameraBox{camera: Box::new(camera)}
//     }
//
//     pub fn deref<T>
// }

pub struct CameraBox {
    pub camera: Box<dyn kiss3d::camera::Camera + Sync + Send>,
}

pub type EntityQueue = Vec<crate::entities::EntityType>;
pub type EntityRemovalQueue = Vec<specs::Entity>;
