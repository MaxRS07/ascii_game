use crate::game::*;
use crate::utils::*;
use minifb::{Key, Window, WindowOptions};
use vek::*;

pub fn run_window(game: &mut Game) {
    let (width, height) = (game.width, game.height);
    let mut opts = WindowOptions::default();
    opts.scale = minifb::Scale::X8;
    let mut window = Window::new("ASCII Game", width, height, opts).unwrap_or_else(|e| {
        panic!("{}", e);
    });
    // Limit to max ~60 fps update rate
    window.set_target_fps(16600);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        handle_pressed_keys(game, &window);
        let buffer = game.draw_scene();
        window.update_with_buffer(&buffer, width, height).unwrap();
    }
}
pub fn handle_pressed_keys(game: &mut Game, window: &Window) {
    window
        .get_keys_pressed(minifb::KeyRepeat::Yes)
        .iter()
        .for_each(|key| match key {
            _ => (),
        });
    let mut mult = 1.0;
    window
        .get_keys_pressed(minifb::KeyRepeat::Yes)
        .iter()
        .for_each(|key| match key {
            Key::W => game.player.pos += game.player.direction * 0.1f32,
            Key::S => game.player.pos += game.player.direction * -0.1f32,
            Key::A => game.player.pos += game.player.direction.cross(Vec3::unit_y()),
            Key::D => game.player.pos += game.player.direction.cross(-Vec3::unit_y()),
            Key::Right => game.player.direction = game.player.direction.rotate(Vec3::unit_y(), 0.1),
            _ => {}
        });
    println!("pos: {}, dir: {}", game.player.pos, game.player.direction)
}
