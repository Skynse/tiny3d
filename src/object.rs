use std::path;

use glam::{vec2, vec4, Mat4, Vec2, Vec3, Vec4};

#[derive(Debug, Clone)]
pub struct Object3D {
    pub vertices: Vec<Vec3>,
    pub indices: Vec<usize>,
    pub position: Vec3,
    pub rotation: Vec3,
    pub scale: Vec3,
}

impl Object3D {
    pub fn new(vertices: Vec<Vec3>, indices: Vec<usize>) -> Self {
        Self {
            vertices,
            indices,
            position: Vec3::ZERO,
            rotation: Vec3::ZERO,
            scale: Vec3::ONE,
        }
    }

    pub fn from_obj(path: &str) -> Object3D {
        let obj_file = path::Path::new(path);
        let obj_file = std::fs::read_to_string(obj_file).unwrap();
        let mut vertices = vec![];
        let mut indices = vec![];

        for line in obj_file.lines() {
            let line = line.trim();
            if line.starts_with("v ") {
                let vertex: Vec<f32> = line
                    .split_whitespace()
                    .skip(1)
                    .map(|x| x.parse().unwrap())
                    .collect();
                vertices.push(Vec3::new(vertex[0], vertex[1], vertex[2]));
            } else if line.starts_with("f ") {
                let face: Vec<usize> = line
                    .split_whitespace()
                    .skip(1)
                    .map(|x| x.split('/').next().unwrap().parse().unwrap())
                    .collect();
                indices.push(face[0] - 1);
                indices.push(face[1] - 1);
                indices.push(face[2] - 1);
            }
        }

        Object3D::new(vertices, indices)
    }
}
