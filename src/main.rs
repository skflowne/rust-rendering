use renderer3d::{prelude::*, Camera, Mesh};
use vecx::{Vec2, Vec3};

pub fn main() {
    let mut eng = Engine::build(EngineConfig::new(EngineConfigParams {
        window_title: Some("3d Renderer".to_string()),
        ..EngineConfigParams::default()
    }));

    //let points = build_cube();

    let fov = 640.0;
    let cam_pos = Vec3(0.0, 0.0, -5.0);

    //let mut cube = Mesh::cube();
    let mut cube = Mesh::load_obj("./assets/f22.obj").unwrap();
    let camera = Camera::new(cam_pos, fov);

    println!("Start update");
    eng.on_update(&mut |eng| {
        eng.draw_grid(10, Some(0xFF333333));
        cube.transform.rotation += Vec3(0.01, 0.01, 0.0);

        let projected_vertices: Vec<Vec2> = cube
            .triangles()
            .flatten()
            .map(|vertex| {
                let transformed = vertex.rot(cube.transform.rotation);
                let projected = camera.project(&transformed);
                let centered = Vec2(
                    projected.x() + eng.config().width() as f64 / 2.0,
                    projected.y() + eng.config().height() as f64 / 2.0,
                );
                return centered;
            })
            .collect();

        projected_vertices.chunks(3).for_each(|tri| {
            let a = tri[0];
            let b = tri[1];
            let c = tri[2];
            //eng.draw_rect(a.x() as usize, a.y() as usize, 4, 4, 0xFF00FF00);
            //eng.draw_rect(b.x() as usize, b.y() as usize, 4, 4, 0xFF00FF00);
            //eng.draw_rect(c.x() as usize, c.y() as usize, 4, 4, 0xFF00FF00);

            eng.draw_line(a.x(), a.y(), b.x(), b.y(), 0xFF00FF00);
            eng.draw_line(b.x(), b.y(), c.x(), c.y(), 0xFF00FF00);
            eng.draw_line(c.x(), c.y(), a.x(), a.y(), 0xFF00FF00);
        });

        /*for face in cube.faces {

        }*/

        /*projected_vertices.iter().for_each(|point| {
            let x = point.x();
            let y = point.y();

            eng.draw_rect(x as usize, y as usize, 4, 4, 0xFFFF0000);
            eng.draw_line(x0, y0, x1, y1, 0xFFFF0000);
        });*/
    });
}
