use std::{collections::binary_heap::Iter, vec};

use ::vecx::Vec3;

const CUBE_VERTS: [Vec3; 8] = [
    // front
    Vec3(-1.0, -1.0, -1.0),
    Vec3(-1.0, 1.0, -1.0),
    Vec3(1.0, 1.0, -1.0),
    Vec3(1.0, -1.0, -1.0),
    // back
    Vec3(-1.0, -1.0, 1.0),
    Vec3(-1.0, 1.0, 1.0),
    Vec3(1.0, 1.0, 1.0),
    Vec3(1.0, -1.0, 1.0),
];

const CUBE_FACES: [Triangle; 6 * 2] = [
    // front
    Triangle(1, 2, 3),
    Triangle(1, 3, 4),
    // right
    Triangle(4, 3, 5),
    Triangle(4, 5, 6),
    // left
    Triangle(8, 7, 2),
    Triangle(8, 2, 1),
    // back
    Triangle(6, 5, 7),
    Triangle(6, 7, 8),
    // top
    Triangle(2, 7, 5),
    Triangle(2, 5, 3),
    // bottom
    Triangle(6, 8, 1),
    Triangle(6, 1, 4),
];

#[derive(Debug, Clone, Copy)]
pub struct Triangle(pub usize, pub usize, pub usize);

pub struct Mesh {
    pub vertices: Vec<Vec3>,
    pub faces: Vec<Triangle>,
}

pub struct FaceIter<'a> {
    current: usize,
    mesh: &'a Mesh,
}

impl<'a> FaceIter<'a> {
    pub fn new(mesh: &'a Mesh) -> Self {
        FaceIter { current: 0, mesh }
    }
}

impl<'a> Iterator for FaceIter<'a> {
    type Item = Vec<Vec3>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.mesh.faces.len() {
            let tri = self.mesh.faces[self.current];
            let v1 = self.mesh.vertices[tri.0 - 1];
            let v2 = self.mesh.vertices[tri.1 - 1];
            let v3 = self.mesh.vertices[tri.2 - 1];

            self.current += 1;
            return Some(vec![v1, v2, v3]);
        }

        None
    }
}

impl Mesh {
    pub fn cube() -> Self {
        Self::new(&CUBE_VERTS, &CUBE_FACES)
    }

    pub fn new(vertices: &[Vec3], faces: &[Triangle]) -> Self {
        Mesh {
            vertices: vertices.iter().map(|v| v.clone()).collect(),
            faces: faces.iter().map(|t| t.clone()).collect(),
        }
    }

    pub fn face_vertices(&self) -> FaceIter {
        FaceIter::new(self)
    }
}
