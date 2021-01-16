use std::cell::{RefCell, RefMut};
use std::collections::HashMap;
use std::rc::Rc;

use kiss3d::scene::SceneNode;
use kiss3d::window::Window;

pub struct GameContext {
    window: Window,
    gfx_manager: HashMap<u32, Rc<RefCell<SceneNode>>>,
    last_assigned_id: u32,
}

impl GameContext {
    pub fn window_mut(&mut self) -> &mut Window {
        &mut self.window
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn new(window: Window) -> Self {
        GameContext {
            window,
            gfx_manager: HashMap::new(),
            last_assigned_id: 0,
        }
    }

    pub fn store_gfx(&mut self, node: SceneNode) -> u32 {
        self.gfx_manager
            .insert(self.last_assigned_id, Rc::new(RefCell::new(node)));
        self.last_assigned_id += 1;
        return self.last_assigned_id - 1;
    }

    pub fn remove_gfx(&mut self, node_id: u32) {
        let ref mut node = self
            .gfx_manager
            .get(&node_id)
            .expect("could not find node")
            .borrow_mut();
        self.window.remove_node(node);
    }

    pub fn get_gfx(&mut self, node_id: u32) -> RefMut<SceneNode> {
        self.gfx_manager
            .get(&node_id)
            .expect("could not find node")
            .borrow_mut()
    }
}
