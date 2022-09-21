use vecx::{Vec2, Vec3};

pub struct Camera {
    fov: f64,
    position: Vec3,
}

impl Camera {
    pub fn new(position: Vec3, fov: f64) -> Self {
        Camera { fov, position }
    }

    pub fn project(&self, point: &Vec3) -> Vec2 {
        let z = point.z() - self.position.z();
        let mut x = point.x() / z;
        let mut y = point.y() / z;
        x *= self.fov;
        y *= self.fov;

        Vec2(x, y)
    }
}
