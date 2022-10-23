use vecx::{Matrix, Vec3, Vec4};

pub enum ProjectionType {
    Perspective,
}

pub struct CameraProjection {
    projection_type: ProjectionType,
    projection_matrix: Matrix,
    aspect_ratio: f64,
    fov: f64,
    z_near: f64,
    z_far: f64,
}

impl CameraProjection {
    pub fn perspective(aspect_ratio: f64, fov: f64, z_near: f64, z_far: f64) -> Self {
        let mut projection_matrix = Matrix::sqr4();

        let a = aspect_ratio;
        let f = 1.0 / f64::tan(fov / 2.0);
        let d = z_far / (z_far - z_near);

        projection_matrix.set((1, 1), a * f);
        projection_matrix.set((2, 2), f);
        projection_matrix.set((3, 3), d);
        projection_matrix.set((3, 4), -(z_far * z_near) / (z_far - z_near));
        projection_matrix.set((4, 3), 1.0);

        CameraProjection {
            aspect_ratio,
            fov,
            z_near,
            z_far,
            projection_type: ProjectionType::Perspective,
            projection_matrix,
        }
    }

    pub fn projection_type(&self) -> &ProjectionType {
        &self.projection_type
    }

    pub fn fov(&self) -> f64 {
        self.fov
    }

    pub fn aspect_ratio(&self) -> f64 {
        self.aspect_ratio
    }

    pub fn z_near(&self) -> f64 {
        self.z_near
    }

    pub fn z_far(&self) -> f64 {
        self.z_far
    }

    pub fn project(&self, point: &Vec3) -> Vec4 {
        let point_mat = point.as_mat4(1.0);
        let proj_matrix = &self.projection_matrix;
        Vec4::from(proj_matrix * &point_mat)
    }
}

pub struct Camera {
    position: Vec3,
    projection: CameraProjection,
}

impl Camera {
    pub fn new(position: Vec3, projection: CameraProjection) -> Self {
        Camera {
            position,
            projection,
        }
    }

    pub fn position(&self) -> &Vec3 {
        &self.position
    }

    pub fn project(&self, point: &Vec3) -> Vec4 {
        let mut projected = self.projection.project(point);

        if projected.w() != 0.0 {
            projected /= Vec4::from(projected.w());
        }

        return projected;
    }
}
