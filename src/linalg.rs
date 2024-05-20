// Helper functions for rotation matrices
extern crate glam;
use glam::{vec2, vec3, vec4, Mat4, Vec2, Vec3, Vec4};
use macroquad::prelude::*;

use crate::object::Object3D;
pub fn rotate_x(angle: f32) -> Mat4 {
    let angle = angle.to_radians();
    Mat4::from_cols(
        vec4(1.0, 0.0, 0.0, 0.0),
        vec4(0.0, angle.cos(), -angle.sin(), 0.0),
        vec4(0.0, angle.sin(), angle.cos(), 0.0),
        vec4(0.0, 0.0, 0.0, 1.0),
    )
}

pub fn rotate_y(angle: f32) -> Mat4 {
    let angle = angle.to_radians();
    Mat4::from_cols(
        vec4(angle.cos(), 0.0, angle.sin(), 0.0),
        vec4(0.0, 1.0, 0.0, 0.0),
        vec4(-angle.sin(), 0.0, angle.cos(), 0.0),
        vec4(0.0, 0.0, 0.0, 1.0),
    )
}

pub fn rotate_z(angle: f32) -> Mat4 {
    let angle = angle.to_radians();
    Mat4::from_cols(
        vec4(angle.cos(), -angle.sin(), 0.0, 0.0),
        vec4(angle.sin(), angle.cos(), 0.0, 0.0),
        vec4(0.0, 0.0, 1.0, 0.0),
        vec4(0.0, 0.0, 0.0, 1.0),
    )
}

fn custom_rand(start: usize, end: usize) -> usize {
    let datetime = crate::miniquad::date::now();
    let seed = datetime as usize;

    let mut x = seed;

    x ^= x << 13;

    x ^= x >> 17;
    x ^= x << 5;

    x % (end - start) + start
}

pub fn draw_wireframe_edges(a: Vec2, b: Vec2, c: Vec2) {
    draw_line(a.x, a.y, b.x, b.y, 2.0, WHITE);
    draw_line(b.x, b.y, c.x, c.y, 2.0, WHITE);
    draw_line(c.x, c.y, a.x, a.y, 2.0, WHITE);
}
// Convert normalized device coordinates to screen space coordinates
pub fn ndc_to_screen_space(ndc: Vec2) -> Vec2 {
    vec2(
        (ndc.x * 0.5 + 0.5) * screen_width(),
        (ndc.y * -0.5 + 0.5) * screen_height(),
    )
}

pub fn calculate_normal(v0: Vec3, v1: Vec3, v2: Vec3) -> Vec3 {
    let u = v2 - v0; // Note the order: v2 - v0
    let v = v1 - v0; // Note the order: v1 - v0
    u.cross(v)
}

pub fn fill_triangle(a: Vec2, mut b: Vec2, mut c: Vec2, color: Color) {
    // smooth shading
    let mut a = a;
    let mut b = b;
    let mut c = c;

    if a.y > b.y {
        std::mem::swap(&mut a, &mut b);
    }
    if a.y > c.y {
        std::mem::swap(&mut a, &mut c);
    }
    if b.y > c.y {
        std::mem::swap(&mut b, &mut c);
    }

    let total_height = c.y - a.y;

    for i in 0..total_height as i32 {
        let second_half = i > b.y as i32 - a.y as i32 || b.y == a.y;
        let segment_height = if second_half { c.y - b.y } else { b.y - a.y };

        let alpha = i as f32 / total_height;
        let beta = if second_half {
            (i - b.y as i32) as f32 / segment_height
        } else {
            (i - a.y as i32) as f32 / segment_height
        };

        let mut a = a + (c - a) * alpha;
        let mut b = if second_half {
            b + (c - b) * beta
        } else {
            a + (b - a) * beta
        };

        if a.x > b.x {
            std::mem::swap(&mut a, &mut b);
        }

        for j in a.x as i32..b.x as i32 {
            let phi = if b.x - a.x > 0.0 {
                (j as f32 - a.x) / (b.x - a.x)
            } else {
                1.0
            };

            let p = a + (b - a) * phi;
            let idx = (p.x as i32 + p.y as i32 * screen_width() as i32) as usize;
            if idx < screen_width() as usize * screen_height() as usize {
                let mut color = color;
                color.a = 255.0;
                draw_pixel(j, i, color);
            }
        }
    }
}

fn draw_pixel(x: i32, y: i32, color: Color) {
    let x = x as f32;
    let y = y as f32;
    draw_rectangle(x, y, 1.0, 1.0, color);
}

trait PerpDot {
    fn perp_dot(self, other: Self) -> f32;
}

impl PerpDot for Vec2 {
    fn perp_dot(self, other: Self) -> f32 {
        self.x * other.y - self.y * other.x
    }
}

pub fn compute_vertex_normals(object: &mut Object3D) {
    object.normals = vec![Vec3::ZERO; object.vertices.len()];

    for i in (0..object.indices.len()).step_by(3) {
        let index_a = object.indices[i] as usize;
        let index_b = object.indices[i + 1] as usize;
        let index_c = object.indices[i + 2] as usize;

        let a = object.vertices[index_a];
        let b = object.vertices[index_b];
        let c = object.vertices[index_c];

        let normal = calculate_normal(a, b, c);

        object.normals[index_a] += normal;
        object.normals[index_b] += normal;
        object.normals[index_c] += normal;
    }

    for normal in &mut object.normals {
        *normal = normal.normalize();
    }
}
