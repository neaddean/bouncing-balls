use nphysics3d::object::{DefaultBodyHandle, DefaultColliderHandle};
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
pub struct Physical {
    pub body_handle: DefaultBodyHandle,
    pub collider_handle: DefaultColliderHandle,
}
