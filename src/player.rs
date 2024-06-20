use crate::game::*;
use crate::game_object::GameObject;
use crate::physics::*;
use crate::utils::{deccol, enccol_rgb, enccol_rgb_vec3};
use num_traits::Pow;
use vek::*;

pub struct Player {
    pub pos: Vec3<f32>,
    pub direction: Vec3<f32>,
}
impl Player {
    pub fn new(pos: Vec3<f32>) -> Self {
        let direction = Vec3::unit_z();
        Player { pos, direction }
    }
    pub fn trace(&self, buffer: &mut Vec<u32>, game: &Game) {
        let rddist = 20.0;
        let vpd = 1.0;
        let (width, height) = (game.width as u32, game.height as u32);
        let (half_width, half_height) = (width as f32 / 2f32, height as f32 / 2f32);
        for i in 0..width {
            for j in 0..height {
                let j1 = (height - 1) - j;
                for k in [
                    (0.25f32, -0.25f32),
                    (-0.25f32, -0.25f32),
                    (-0.25f32, 0.25f32),
                    (0.25f32, 0.25f32),
                ] {
                    let pos = Vec3::new(
                        i as f32 + k.0 - half_width,
                        j as f32 + k.1 - half_height,
                        self.pos.z + vpd,
                    );
                    let dir = (pos - self.pos).normalized();
                    let ray = Ray::new(
                        self.pos + Vec3::new(i as f32 - half_width, j as f32 - half_height, 0.0),
                        dir,
                    );

                    for triangle in &game.triangles {
                        if let Some(hit_info) = triangle.colliding(ray) {
                            if hit_info.phit.z - self.pos.z < rddist {
                                let mut color = deccol(hit_info.material.color);
                                color /= (1.0 + (hit_info.phit.z - self.pos.z).pow(2f32)) as u8;
                                buffer[(j1 * width + i) as usize] = enccol_rgb_vec3(color);
                                break;
                            }
                        }
                    }
                }
            }
        }
    }
}
