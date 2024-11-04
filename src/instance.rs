use crate::prelude::*;
use kurbo::Vec2;
use std::collections::HashMap;
use vello::Scene;

#[derive(Resource, Default)]
pub struct Instance {
    pub max_scene_id: usize,
    pub scenes: HashMap<usize, Scene>,
    pub scene_names: HashMap<String, usize>,
    asset_server: AssetServer,
    resolution: Vec2,
}

impl Instance {
    pub fn new_scene(&mut self, name: &str) -> Option<&mut Scene> {
        self.max_scene_id += 1;
        self.scenes.insert(self.max_scene_id, Scene::new());
        self.scene_names.insert(name.to_string(), self.max_scene_id);

        self.scenes.get_mut(&self.max_scene_id)
    }

    pub fn get_scene(&mut self, name: &str) -> Option<&mut Scene> {
        let ptr = self.scene_names.get(name);

        match ptr {
            Some(p) => self.scenes.get_mut(p),
            None => None,
        }
    }

    pub fn asset_server(&mut self) -> &mut AssetServer {
        &mut self.asset_server
    }

    pub fn resolution(&self) -> &Vec2 {
        &self.resolution
    }

    pub fn set_resolution(&mut self, x: u32, y: u32) {
        self.resolution = Vec2::new(x as f64, y as f64);
    }
}

pub fn bella_instance_reset(mut root: ResMut<Instance>) {
    #[allow(clippy::for_kv_map)]
    for (_id, scene) in &mut root.scenes {
        scene.reset();
    }
}
