use std::error::Error;
use std::{fs, vec};

use ::vecx::{Vec3, VecX};
use vecx::Matrix;

use crate::Camera;

const CUBE_VERTS: [Vec3; 8] = [
    Vec3(-1.0, -1.0, -1.0), // 1
    Vec3(-1.0, 1.0, -1.0),  // 2
    Vec3(1.0, 1.0, -1.0),   // 3
    Vec3(1.0, -1.0, -1.0),  // 4
    Vec3(1.0, 1.0, 1.0),    // 5
    Vec3(1.0, -1.0, 1.0),   // 6
    Vec3(-1.0, 1.0, 1.0),   // 7
    Vec3(-1.0, -1.0, 1.0),  // 8
];

const CUBE_FACES: [Face; 6 * 2] = [
    // front
    Face(1, 2, 3, 0xFFFF0000),
    Face(1, 3, 4, 0xFFFF0000),
    // right
    Face(4, 3, 5, 0xFF00FF00),
    Face(4, 5, 6, 0xFF00FF00),
    // back
    Face(6, 5, 7, 0xFF0000FF),
    Face(6, 7, 8, 0xFF0000FF),
    // left
    Face(8, 7, 2, 0xFFFFFF00),
    Face(8, 2, 1, 0xFFFFFF00),
    // top
    Face(2, 7, 5, 0xFF00FFFF),
    Face(2, 5, 3, 0xFF00FFFF),
    // bottom
    Face(6, 8, 1, 0xFFFF00FF),
    Face(6, 1, 4, 0xFFFF00FF),
];

#[derive(Debug, Clone, Copy)]
pub struct Face(pub usize, pub usize, pub usize, pub u32);

impl Face {
    pub fn color(&self) -> u32 {
        self.3
    }

    pub fn set_color(&mut self, color: u32) {
        self.3 = color;
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Triangle(pub Vec3, pub Vec3, pub Vec3, pub u32);

impl Triangle {
    /// Returns the first point of this triangle
    pub fn a(&self) -> Vec3 {
        self.0
    }

    /// Returns the second point of this triangle
    pub fn b(&self) -> Vec3 {
        self.1
    }

    /// Returns the third point of this triangle
    pub fn c(&self) -> Vec3 {
        self.2
    }

    pub fn color(&self) -> u32 {
        self.3
    }

    pub fn set_color(&mut self, color: u32) {
        self.3 = color;
    }

    pub fn avg_z(&self) -> f64 {
        (self.0.z() + self.1.z() + self.2.z()) / 3.0
    }

    pub fn projected(&self, cam: &Camera) -> Triangle {
        Triangle(
            Vec3::from(cam.project(&self.a())),
            Vec3::from(cam.project(&self.b())),
            Vec3::from(cam.project(&self.c())),
            self.color(),
        )
    }

    /// Only applies rotation and scale TODO: implement translation
    pub fn transformed(&self, transform: &Transform) -> Self {
        let mut transformed_tri = self.scale(transform.scale);
        transformed_tri = transformed_tri.rotate(transform.rotation);
        transformed_tri = transformed_tri.translate(transform.position);
        transformed_tri
    }

    pub fn matrix_transform(&self, transform: &Transform) -> Self {
        let scale_matrix = Matrix::m4_scale(transform.scale);
        let rot_x_matrix = Matrix::m4_rotate_x(transform.rotation.x());
        let rot_y_matrix = Matrix::m4_rotate_y(transform.rotation.y());
        let rot_z_matrix = Matrix::m4_rotate_z(transform.rotation.z());
        let translation_matrix = Matrix::m4_translate(transform.position);

        //println!("Tri self {:?}", self);

        //translation_mat * rot_z_mat * rot_y_mat * rot_x_mat * scale_mat;
        let mut world_matrix: Matrix = Matrix::id4();
        world_matrix = scale_matrix * world_matrix;
        world_matrix = rot_z_matrix * world_matrix;
        world_matrix = rot_y_matrix * world_matrix;
        world_matrix = rot_x_matrix * world_matrix;
        world_matrix = translation_matrix * world_matrix;

        let transformed = Triangle(
            Vec3::from(&world_matrix * &self.0.as_mat4(1.0)),
            Vec3::from(&world_matrix * &self.1.as_mat4(1.0)),
            Vec3::from(&world_matrix * &self.2.as_mat4(1.0)),
            self.color(),
        );

        //let transformed = Triangle(self.0, self.1, self.2, self.color());

        //println!("Tri outT {:?}", transformed);
        transformed
    }

    pub fn translate(&self, translation: Vec3) -> Triangle {
        Triangle(
            self.0 + translation,
            self.1 + translation,
            self.2 + translation,
            self.color(),
        )
    }

    pub fn scale(&self, scale: Vec3) -> Triangle {
        Triangle(self.0 * scale, self.1 * scale, self.2 * scale, self.color())
    }

    pub fn rotate(&self, rotation: Vec3) -> Triangle {
        Triangle(
            self.0.rot(&rotation),
            self.1.rot(&rotation),
            self.2.rot(&rotation),
            self.color(),
        )
    }

    pub fn normal(&self) -> Vec3 {
        if self.a() == self.b() || self.b() == self.c() {
            return Vec3::zero();
        }

        let ab = (self.b() - self.a()).normalized();
        let ac = (self.c() - self.a()).normalized();

        return ac.cross(&ab).normalized();
    }

    pub fn should_cull(&self, viewer_position: Vec3) -> bool {
        let normal = self.normal();
        if normal == Vec3::ZERO {
            return true;
        }

        let tri_to_viewer = (viewer_position - self.a()).normalized();

        return normal.dot(&tri_to_viewer) < 0.0;
    }
}

impl IntoIterator for Triangle {
    type IntoIter = TriangleIter;
    type Item = Vec3;
    fn into_iter(self) -> Self::IntoIter {
        TriangleIter {
            current: 0,
            vertices: vec![self.0, self.1, self.2],
        }
    }
}

impl FromIterator<Vec3> for Triangle {
    fn from_iter<T: IntoIterator<Item = Vec3>>(iter: T) -> Self {
        let mut iterator = iter.into_iter();
        Triangle(
            iterator.next().unwrap(),
            iterator.next().unwrap(),
            iterator.next().unwrap(),
            0xFFFFFFF,
        )
    }
}

pub struct TriangleIter {
    current: usize,
    vertices: Vec<Vec3>,
}

impl Iterator for TriangleIter {
    type Item = Vec3;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < 3 {
            let vertex = self.vertices[self.current];
            self.current += 1;
            return Some(vertex);
        }

        None
    }
}

pub struct Transform {
    pub position: Vec3,
    pub rotation: Vec3,
    pub scale: Vec3,
}

impl Default for Transform {
    fn default() -> Self {
        Transform {
            position: Vec3::zero(),
            rotation: Vec3::zero(),
            scale: Vec3::one(),
        }
    }
}

pub struct Mesh {
    pub vertices: Vec<Vec3>,
    pub faces: Vec<Face>,
    pub transform: Transform,
}

pub struct MeshIter<'a> {
    current: usize,
    mesh: &'a Mesh,
}

impl<'a> MeshIter<'a> {
    pub fn new(mesh: &'a Mesh) -> Self {
        MeshIter { current: 0, mesh }
    }
}

impl<'a> Iterator for MeshIter<'a> {
    type Item = Triangle;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.mesh.faces.len() {
            let face = self.mesh.faces[self.current];
            let v1 = self.mesh.vertices[face.0 - 1];
            let v2 = self.mesh.vertices[face.1 - 1];
            let v3 = self.mesh.vertices[face.2 - 1];

            self.current += 1;
            return Some(Triangle(v1, v2, v3, face.color()));
        }

        None
    }
}

impl Mesh {
    pub fn cube() -> Self {
        Self::new(&CUBE_VERTS, &CUBE_FACES)
    }

    pub fn new(vertices: &[Vec3], faces: &[Face]) -> Self {
        Mesh {
            vertices: vertices.iter().map(|v| v.clone()).collect(),
            faces: faces.iter().map(|t| t.clone()).collect(),
            transform: Transform {
                position: Vec3(0.0, 0.0, 0.0),
                rotation: Vec3(0.0, 0.0, 0.0),
                scale: Vec3(1.0, 1.0, 1.0),
            },
        }
    }

    pub fn load_obj(path: &str) -> Result<Self, Box<dyn Error>> {
        let contents = fs::read_to_string(path)?;
        let vertices: Vec<Vec3> = contents
            .lines()
            .filter(|line| line.contains("v ") && !line.contains("#"))
            .map(|line| {
                let coords: Vec3 = line
                    .split_whitespace()
                    .filter(|coord| !coord.contains("v"))
                    .map(|coord| {
                        let coord: f64 = coord.parse().unwrap();
                        return coord;
                    })
                    .collect();

                return Vec3::from(coords);
            })
            .collect();

        let faces: Vec<Face> = contents
            .lines()
            .filter(|line| line.contains("f ") && !line.contains("#"))
            .map(|line| line.replace("f ", ""))
            .map(|line| {
                let faces: Vec<String> = line
                    .split_whitespace()
                    .into_iter()
                    .map(|face| {
                        let index: Vec<String> =
                            face.split("/").take(1).map(|i| String::from(i)).collect();
                        return index[0].clone();
                    })
                    .collect();
                return faces;
            })
            .map(|face| {
                let face_indices: Vec<usize> = face
                    .iter()
                    .map(|index| {
                        //println!("index {:?}", index);
                        let index: usize = index.parse().unwrap();
                        return index;
                    })
                    .collect();

                return Face(
                    face_indices[0],
                    face_indices[1],
                    face_indices[2],
                    rand::random(),
                );
            })
            .collect();

        /*faces = faces
        .chunks(2)
        .map(|faces| {
            let color: u32 = rand::random();
            let mut f1 = Face::from(faces[0]);
            let mut f2 = Face::from(faces[1]);

            f1.set_color(color);
            f2.set_color(color);
            return [f1, f2];
        })
        .flatten()
        .collect();*/
        //println!("vertices {:?}", vertices);
        //println!("faces: {:?}", faces);

        Ok(Mesh {
            vertices,
            faces,
            transform: Transform::default(),
        })
    }

    pub fn triangles(&self) -> MeshIter {
        MeshIter::new(self)
    }

    pub fn transform(&mut self) -> &mut Transform {
        &mut self.transform
    }
}
