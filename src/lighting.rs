use vecx::{Vec3, VecX};

use crate::Triangle;

pub struct GlobalLight {
    direction: Vec3,
}

impl GlobalLight {
    const ALPHA_MASK: u32 = 0xFF000000;
    const RED_MASK: u32 = 0x00FF0000;
    const GREEN_MASK: u32 = 0x0000FF00;
    const BLUE_MASK: u32 = 0x000000FF;

    pub fn new(direction: Vec3) -> Self {
        GlobalLight {
            direction: Vec3(direction.0, -direction.1, -direction.2),
        }
    }

    pub fn light_factor(&self, face_normal: &Vec3) -> f64 {
        f64::min(0.0, self.direction.dot(face_normal)) * -1.0
    }

    pub fn lit_color(&self, original_color: u32, light_factor: f64) -> u32 {
        let a: u32 = original_color & Self::ALPHA_MASK;
        let r: u32 = ((original_color & Self::RED_MASK) as f64 * light_factor) as u32;
        let g: u32 = ((original_color & Self::GREEN_MASK) as f64 * light_factor) as u32;
        let b: u32 = ((original_color & Self::BLUE_MASK) as f64 * light_factor) as u32;

        let lit_color = a | (r & Self::RED_MASK) | (g & Self::GREEN_MASK) | (b & Self::BLUE_MASK);
        return lit_color;
    }

    pub fn shaded_triangle(&self, triangle: &Triangle) -> Triangle {
        let light_factor = self.light_factor(&triangle.normal());
        let color = self.lit_color(triangle.color(), light_factor);
        Triangle(triangle.0, triangle.1, triangle.2, color)
    }
}
