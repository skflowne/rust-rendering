use renderer3d::prelude::*;
use vecx::Vec3;

pub fn main() {
    let mut eng = Engine::build(EngineConfig {
        window_title: "3d Renderer".to_string(),
        width: 800,
        height: 600,
        clear_color: 0xFF000000,
    });

    let points = build_cube();

    let fov = 640.0;
    let cam_pos = Vec3(0.0, 0.0, -5.0);
    let mut rotation = Vec3(0.0, 0.0, 0.0);

    println!("Start update");
    eng.on_update(&mut |eng: &mut Engine| {
        eng.draw_grid(10, Some(0xFF333333));
        rotation = rotation + Vec3(0.01, 0.02, 0.0);

        let mut points = transform_points(&points, rotation);
        points = project_points(&points, cam_pos, fov);

        points.iter().for_each(|point| {
            let mut x = point.x();
            let mut y = point.y();

            x += eng.config().width as f64 / 2.0;
            y += eng.config().height as f64 / 2.0;

            eng.draw_rect(x as usize, y as usize, 4, 4, 0xFFFF0000);
        });
    });
}

fn build_cube() -> Vec<Vec3> {
    const CUBE_SIZE: usize = 9;
    const NUM_POINTS: usize = CUBE_SIZE * CUBE_SIZE * CUBE_SIZE;
    let mut points = vec![Vec3(0.0, 0.0, 0.0); NUM_POINTS];

    let mut i = 0;

    let step = 25;
    for x in (-100..=100).step_by(step).map(|x| x as f64 * 0.01) {
        for y in (-100..=100).step_by(step).map(|y| y as f64 * 0.01) {
            for z in (-100..=100).step_by(step).map(|z| z as f64 * 0.01) {
                let point = Vec3(x, y, z);
                println!("p {}: {}", i, (point.x() / point.z()));
                points[i] = point;
                i += 1;
            }
        }
    }

    points
}

fn transform_points(points: &Vec<Vec3>, rotation: Vec3) -> Vec<Vec3> {
    points.iter().map(|p| p.rot(rotation)).collect()
}

fn project_points(points: &Vec<Vec3>, cam_pos: Vec3, fov: f64) -> Vec<Vec3> {
    points
        .iter()
        .map(|point| {
            let z = point.z() + cam_pos.z();
            let mut x = point.x() / z;
            let mut y = point.y() / z;
            x *= fov;
            y *= fov;

            Vec3(x, y, z)
        })
        .collect()
}
