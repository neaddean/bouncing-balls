use nalgebra::{Isometry3, Vector3};
use specs::{Component, VecStorage};

#[derive(Component)]
#[storage(VecStorage)]
pub struct Renderable {
    pub gfx_id: u32,
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Ball {
    pub radius: f32,
}

#[derive(Component, Copy, Clone)]
#[storage(VecStorage)]
pub struct Position {
    pub position: Isometry3<f32>,
}

#[derive(Component, Copy, Clone)]
#[storage(VecStorage)]
pub struct Velocity {
    pub vector: Vector3<f32>,
}
