use super::renderer::object::Object;
use glam::Vec2;

pub struct Player {
    obj: Object,
    hp: i32,
    pos: Vec2,
    velocity: Vec2
}

impl Player {
    pub fn new(obj: Object) -> Self {
        Self {
            obj,
            hp: 5,
            pos: Vec2::ZERO,
            velocity: Vec2::ZERO,
        }
    }
    pub fn get_obj(&self) -> &Object {
        &self.obj
    }
}
