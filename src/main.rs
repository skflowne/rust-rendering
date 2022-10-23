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
    let mut mesh = Mesh::load_obj("./assets/f22.obj").unwrap();
    let camera = Camera::new(cam_pos, fov);
    let wireframe_color = 0xFF00FF00;

    println!("Start update");
    eng.on_update(&mut |eng| {
        eng.draw_grid(10, Some(0xFF333333));
        mesh.transform.rotation += Vec3(0.01, 0.0, 0.01);
        //mesh.transform.scale += Vec3(0.001, 0.001, 0.001);
        //mesh.transform.position += Vec3(0.01, 0.0, 0.0);

        let mut projected_tris: Vec<Triangle> = mesh
            .triangles()
            .map(|triangle| triangle.matrix_transform(&mesh.transform))
            .filter(|triangle| {
                !eng.config().backface_culling_enabled() || triangle.should_cull(cam_pos)
            })
            .map(|tri| {
                tri.projected(&camera).translate(Vec3::from(Vec2(
                    eng.config().width() as f64 / 2.0,
                    eng.config().height() as f64 / 2.0,
                )))
            })
            .collect();

        projected_tris.sort_by(|a, b| b.avg_z().total_cmp(&a.avg_z()));

        projected_tris.iter().for_each(|tri| {
            let a = tri.a();
            let b = tri.b();
            let c = tri.c();

            match eng.config().render_mode() {
                RenderMode::VerticesWireframe => {
                    eng.draw_rect(a.x() as usize, a.y() as usize, 4, 4, 0xFFFF0000);
                    eng.draw_rect(b.x() as usize, b.y() as usize, 4, 4, 0xFFFF0000);
                    eng.draw_rect(c.x() as usize, c.y() as usize, 4, 4, 0xFFFF0000);

                    eng.draw_line(a.x(), a.y(), b.x(), b.y(), wireframe_color);
                    eng.draw_line(b.x(), b.y(), c.x(), c.y(), wireframe_color);
                    eng.draw_line(c.x(), c.y(), a.x(), a.y(), wireframe_color);
                }
                RenderMode::Wireframe => {
                    eng.draw_line(a.x(), a.y(), b.x(), b.y(), wireframe_color);
                    eng.draw_line(b.x(), b.y(), c.x(), c.y(), wireframe_color);
                    eng.draw_line(c.x(), c.y(), a.x(), a.y(), wireframe_color);
                }
                RenderMode::Solid => {
                    draw_filled_triangle(a.into(), b.into(), c.into(), tri.color(), eng);
                }
                RenderMode::SolidWireframe => {
                    draw_filled_triangle(a.into(), b.into(), c.into(), tri.color(), eng);
                    eng.draw_line(a.x(), a.y(), b.x(), b.y(), wireframe_color);
                    eng.draw_line(b.x(), b.y(), c.x(), c.y(), wireframe_color);
                    eng.draw_line(c.x(), c.y(), a.x(), a.y(), wireframe_color);
                }
            }
        });
    });
}

fn draw_filled_triangle(a: Vec2, b: Vec2, c: Vec2, color: u32, eng: &mut EngineCore) {
    let mut points = vec![a, b, c];
    points.sort_by(|a, b| a.y().partial_cmp(&b.y()).unwrap());
    //println!("points {:?}", points);

    let a = points[0];
    let b = points[1];
    let c = points[2];

    if b.y() == c.y() {
        draw_flat_bottom(a, b, c, color, eng);
    } else if a.y() == b.y() {
        draw_flat_top(a, b, c, color, eng);
    } else {
        let my = b.y();
        let mx = ((c.x() - a.x()) * (b.y() - a.y())) / (c.y() - a.y()) + a.x();

        draw_flat_bottom(a, b, Vec2(mx, my), color, eng);
        draw_flat_top(b, Vec2(mx, my), c, color, eng);
    }
}

fn draw_flat_top(a: Vec2, b: Vec2, c: Vec2, color: u32, eng: &mut EngineCore) {
    let start = c;

    let end1 = a;
    let end2 = b;

    let s1 = (end1.x() - start.x()) / (end1.y() - start.y());
    let s2 = (end2.x() - start.x()) / (end2.y() - start.y());

    let y1 = end1.y().round() as usize;
    let y2 = start.y().round() as usize;

    let mut x1 = end1.x();
    let mut x2 = end2.x();

    for y in y1..y2 {
        eng.draw_line(x1, y as f64, x2, y as f64, color);
        x1 += s1;
        x2 += s2;
    }
}

fn draw_flat_bottom(a: Vec2, b: Vec2, c: Vec2, color: u32, eng: &mut EngineCore) {
    let start = a;

    let end1 = b;
    let end2 = c;

    let s1 = (end1.x() - start.x()) / (end1.y() - start.y());
    let s2 = (end2.x() - start.x()) / (end2.y() - start.y());

    let y1 = start.y().round() as usize;
    let y2 = end1.y().round() as usize;

    let mut x1 = start.x();
    let mut x2 = start.x();

    //println!("draw from {} to {}", y1, y2);
    for y in y1..y2 {
        //println!("draw flat bottom x1{} x2{} y{}", x1, x2, y);
        eng.draw_line(x1, y as f64, x2, y as f64, color);
        x1 += s1;
        x2 += s2;
    }
}
