use nalgebra::Isometry3;
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
