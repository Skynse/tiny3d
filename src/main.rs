extern crate glam;
use glam::{vec2, vec3, vec4, FloatExt, Mat4, Vec2, Vec3, Vec4, Vec4Swizzles};
use macroquad::{prelude::*, time};
use std::vec;

mod linalg;
mod object;
mod render;

use object::Object3D;

// Struct for 3D objects

fn generate_sphere(radius: f32, segments: usize, rings: usize) -> Object3D {
    let mut vertices = vec![];
    let mut indices = vec![];

    for i in 0..segments {
        let theta = i as f32 / segments as f32 * std::f32::consts::PI;
        let cos_theta = theta.cos();
        let sin_theta = theta.sin();

        for j in 0..rings {
            let phi = j as f32 / rings as f32 * 2.0 * std::f32::consts::PI;
            let cos_phi = phi.cos();
            let sin_phi = phi.sin();

            let x = radius * sin_theta * cos_phi;
            let y = radius * cos_theta;
            let z = radius * sin_theta * sin_phi;

            vertices.push(vec3(x, y, z));
        }
    }

    for i in 0..segments {
        for j in 0..rings {
            let a = i * rings + j;
            let b = (i + 1) % segments * rings + j;
            let c = (i + 1) % segments * rings + (j + 1) % rings;
            let d = i * rings + (j + 1) % rings;

            indices.push(a);
            indices.push(b);
            indices.push(c);

            indices.push(a);
            indices.push(c);
            indices.push(d);
        }
    }

    Object3D::new(vertices, indices)
}

fn generate_torus(
    radius: f32,
    tube_radius: f32,
    segments: usize,
    tube_segments: usize,
) -> Object3D {
    let mut vertices = vec![];
    let mut indices = vec![];

    for i in 0..segments {
        let theta = i as f32 / segments as f32 * 2.0 * std::f32::consts::PI;
        let cos_theta = theta.cos();
        let sin_theta = theta.sin();

        for j in 0..tube_segments {
            let phi = j as f32 / tube_segments as f32 * 2.0 * std::f32::consts::PI;
            let cos_phi = phi.cos();
            let sin_phi = phi.sin();

            let x = (radius + tube_radius * cos_phi) * cos_theta;
            let y = tube_radius * sin_phi;
            let z = (radius + tube_radius * cos_phi) * sin_theta;

            vertices.push(vec3(x, y, z));
        }
    }

    for i in 0..segments {
        for j in 0..tube_segments {
            let a = i * tube_segments + j;
            let b = (i + 1) % segments * tube_segments + j;
            let c = (i + 1) % segments * tube_segments + (j + 1) % tube_segments;
            let d = i * tube_segments + (j + 1) % tube_segments;

            indices.push(a);
            indices.push(b);
            indices.push(c);

            indices.push(a);
            indices.push(c);
            indices.push(d);
        }
    }

    Object3D::new(vertices, indices)
}

// gizmo that shows the orientation of the camera

#[macroquad::main("Renderer")]
async fn main() {
    crate::render::run().await;
}

fn float_lerp(lhs: f32, rhs: f32, s: f32) -> f32 {
    lhs + (rhs - lhs) * s
}
