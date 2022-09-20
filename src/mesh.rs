use std::error::Error;
use std::{fs, vec};

use ::vecx::Vec3;

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
    Face(1, 2, 3),
    Face(1, 3, 4),
    // right
    Face(4, 3, 5),
    Face(4, 5, 6),
    // back
    Face(6, 5, 7),
    Face(6, 7, 8),
    // left
    Face(8, 7, 2),
    Face(8, 2, 1),
    // top
    Face(2, 7, 5),
    Face(2, 5, 3),
    // bottom
    Face(6, 8, 1),
    Face(6, 1, 4),
];

#[derive(Debug, Clone, Copy)]
pub struct Face(pub usize, pub usize, pub usize);

#[derive(Debug, Clone, Copy)]
pub struct Triangle(pub Vec3, pub Vec3, pub Vec3);

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

#[derive(Default)]
pub struct Transform {
    pub position: Vec3,
    pub rotation: Vec3,
    pub scale: Vec3,
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
            let tri = self.mesh.faces[self.current];
            let v1 = self.mesh.vertices[tri.0 - 1];
            let v2 = self.mesh.vertices[tri.1 - 1];
            let v3 = self.mesh.vertices[tri.2 - 1];

            self.current += 1;
            return Some(Triangle(v1, v2, v3));
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
                scale: Vec3(0.0, 0.0, 0.0),
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
                        println!("index {:?}", index);
                        let index: usize = index.parse().unwrap();
                        return index;
                    })
                    .collect();

                return Face(face_indices[0], face_indices[1], face_indices[2]);
            })
            .collect();

        println!("vertices {:?}", vertices);
        println!("faces: {:?}", faces);

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
