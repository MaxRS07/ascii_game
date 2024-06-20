use crate::{
    game_object::{self, *},
    utils::enccol_rgb,
};
use rand::*;
use vek::*;

use crate::player::*;
pub struct Game {
    pub width: usize,
    pub height: usize,
    pub triangles: Vec<Triangle>,

    pub player: Player,
}
impl Game {
    pub fn new(width: usize, height: usize) -> Self {
        let player = Player::new(Vec3::zero());
        Game {
            width,
            height,
            triangles: vec![],
            player,
        }
    }
    pub fn draw_scene(&self) -> Vec<u32> {
        let mut buffer = vec![0u32; self.width * self.height];
        self.player.trace(&mut buffer, self);
        buffer
    }
    pub fn push_triangle(&mut self, triangle: Triangle) {
        self.triangles.push(triangle);
    }
    pub fn add_shape(&mut self, shape: &dyn Polygon) {
        for triangle in shape.construct() {
            self.push_triangle(triangle);
        }
    }
}
