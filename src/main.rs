mod game;
mod game_object;
mod physics;
mod player;
mod utils;
mod window;

use std::sync::Arc;

use game::Game;
use game_object::{GameObject, Plane, Triangle};
use physics::Material;
use utils::enccol_rgb;
use vek::*;
use window::run_window;

fn main() {
    let width: usize = 50;
    let height: usize = 50;

    let mut game = Game::new(width, height);
    let red: Material = Material {
        color: enccol_rgb(255, 0, 0),
    };
    let green = Material {
        color: enccol_rgb(0, 255, 0),
    };
    let purple = Material {
        color: enccol_rgb(255, 0, 255),
    };
    let a = Vec3::new(-10.0, 10.0, 10.0);
    let b = Vec3::new(10.0, -10.0, 10.0);
    let c = Vec3::new(-30.0, -10.0, 10.0);
    let d = Vec3::new(10.0, 0.0, 10.0);

    let plane = Plane::new(a, b, red);
    let plane2 = Plane::new(c, a, green);
    let plane3 = Plane::new(d, b, purple);
    let floor = Plane::new(
        Vec3::new(-10.0, 0.0, -10.0),
        Vec3::new(10.0, 0.0, 10.0),
        Material::new(enccol_rgb(0, 0, 255)),
    );
    game.add_shape(&plane);
    game.add_shape(&plane2);
    game.add_shape(&plane3);
    game.add_shape(&floor);
    run_window(&mut game);
}
