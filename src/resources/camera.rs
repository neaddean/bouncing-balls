// use std::borrow::{Borrow, BorrowMut};
// use std::cell::RefCell;
// use std::collections::HashMap;
// use std::rc::Rc;
//
// use kiss3d::camera::Camera;
// use kiss3d::event::Event;
// use specs::{join::Join, ReadStorage, System};
//
// use crate::components::*;
// use crate::context::GameContext;
//
// struct CameraEntry {
//     pub camera: Box<dyn Camera>,
//     pub inhibited: bool,
// }
//
// #[derive(Default)]
// pub struct CameraStack<'a> {
//     cameras: HashMap<String, CameraEntry<'a>>,
//     active_camera: Option<String>,
//     active_planar_camera: Option<String>,
// }
//
// impl CameraStack {
//     // pub fn new() -> Self {
//     //     CameraStack {
//     //         cameras: HashMap::new(),
//     //         active_camera: None,
//     //         active_planar_camera: None,
//     //     }
//     // }
//
//     pub fn set_camera<T>(&mut self, camera: T)
//         where T: Camera
//     {
//         self.cameras.insert("default".to_string(), CameraEntry { camera: Box::new(camera), inhibited: false });
//         self.active_camera = Some("default".to_string());
//     }
//
//     pub fn get_camera<T>(&mut self) -> Option<&mut T>
//         where T: Camera
//     {
//         let mut active_camera = self.cameras.get("default")?;
//         let inhibited = active_camera.inhibited;
//         if !inhibited{
//             return Some(&mut active_camera.camera.borrow_mut())
//         }
//         None
//     }
// }
