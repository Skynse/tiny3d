extern crate glam;
use glam::{vec2, vec3, vec4, Mat4, Vec2, Vec3, Vec4, Vec4Swizzles};
use macroquad::{prelude::*, time};
use std::vec;

mod object;

use object::Object3D;

struct Camera {
    position: Vec3,
    target: Vec3,
    up: Vec3,
}

struct Light {
    direction: Vec3,
    color: Color,
}

struct Scene {
    objects: Vec<Object3D>,
    camera: Camera,
    light: Light,
}
// Struct for 3D objects

// Helper functions for rotation matrices
fn rotate_x(angle: f32) -> Mat4 {
    let angle = angle.to_radians();
    Mat4::from_cols(
        vec4(1.0, 0.0, 0.0, 0.0),
        vec4(0.0, angle.cos(), -angle.sin(), 0.0),
        vec4(0.0, angle.sin(), angle.cos(), 0.0),
        vec4(0.0, 0.0, 0.0, 1.0),
    )
}

fn rotate_y(angle: f32) -> Mat4 {
    let angle = angle.to_radians();
    Mat4::from_cols(
        vec4(angle.cos(), 0.0, angle.sin(), 0.0),
        vec4(0.0, 1.0, 0.0, 0.0),
        vec4(-angle.sin(), 0.0, angle.cos(), 0.0),
        vec4(0.0, 0.0, 0.0, 1.0),
    )
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

fn rotate_z(angle: f32) -> Mat4 {
    let angle = angle.to_radians();
    Mat4::from_cols(
        vec4(angle.cos(), -angle.sin(), 0.0, 0.0),
        vec4(angle.sin(), angle.cos(), 0.0, 0.0),
        vec4(0.0, 0.0, 1.0, 0.0),
        vec4(0.0, 0.0, 0.0, 1.0),
    )
}
fn draw_object(
    object: &Object3D,
    proj_mat: Mat4,
    view_mat: Mat4,
    camera_pos: Vec3,
    light_dir: Vec3,
) {
    let model_mat = Mat4::from_translation(object.position)
        * rotate_x(object.rotation.x)
        * rotate_y(object.rotation.y)
        * rotate_z(object.rotation.z)
        * Mat4::from_scale(object.scale);
    let mv_mat = view_mat * model_mat; // Model-view matrix
    let mvp_mat = proj_mat * mv_mat; // Model-view-projection matrix

    let mut transformed_vertices: Vec<Vec4> = vec![];
    let mut ndc_vertices: Vec<Vec2> = vec![];
    let mut normals: Vec<Vec3> = vec![];

    for vertex in &object.vertices {
        let transformed_vertex = mvp_mat * Vec4::new(vertex.x, vertex.y, vertex.z, 1.0);
        transformed_vertices.push(transformed_vertex);
        ndc_vertices.push(vec2(
            transformed_vertex.x / transformed_vertex.w,
            transformed_vertex.y / transformed_vertex.w,
        ));
    }

    for i in 0..object.indices.len() / 3 {
        let i = i * 3;
        let index_a = object.indices[i];
        let index_b = object.indices[i + 1];
        let index_c = object.indices[i + 2];

        let w_a = transformed_vertices[index_a].w;
        let w_b = transformed_vertices[index_b].w;
        let w_c = transformed_vertices[index_c].w;

        if w_a > 0.0 && w_b > 0.0 && w_c > 0.0 {
            let a = ndc_vertices[index_a];
            let b = ndc_vertices[index_b];
            let c = ndc_vertices[index_c];

            let screen_a = ndc_to_screen_space(a);
            let screen_b = ndc_to_screen_space(b);
            let screen_c = ndc_to_screen_space(c);

            // Calculate normal in model space
            let normal_model = calculate_normal(
                object.vertices[index_a],
                object.vertices[index_b],
                object.vertices[index_c],
            );

            // Transform normal to view space
            let normal_view =
                mv_mat * Vec4::new(normal_model.x, normal_model.y, normal_model.z, 0.0);
            let normal_view = Vec3::new(normal_view.x, normal_view.y, normal_view.z).normalize();

            // Backface culling
            let view_vector = (camera_pos - object.position).normalize();
            if normal_view.dot(view_vector) < 0.0 {
                // Triangle is back-facing, skip rendering
                continue;
            }

            let intensity = normal_view.dot(light_dir).max(0.0);
            let shaded_color = Color::new(intensity, intensity, intensity, 1.0);

            fill_triangle(screen_a, screen_b, screen_c, shaded_color);
            //draw_wireframe_edges(screen_a, screen_b, screen_c);
        }
    }
}

fn draw_wireframe_edges(a: Vec2, b: Vec2, c: Vec2) {
    draw_line(a.x, a.y, b.x, b.y, 2.0, WHITE);
    draw_line(b.x, b.y, c.x, c.y, 2.0, WHITE);
    draw_line(c.x, c.y, a.x, a.y, 2.0, WHITE);
}
// Convert normalized device coordinates to screen space coordinates
fn ndc_to_screen_space(ndc: Vec2) -> Vec2 {
    vec2(
        (ndc.x * 0.5 + 0.5) * screen_width(),
        (ndc.y * -0.5 + 0.5) * screen_height(),
    )
}

fn calculate_normal(v0: Vec3, v1: Vec3, v2: Vec3) -> Vec3 {
    let u = v2 - v0; // Note the order: v2 - v0
    let v = v1 - v0; // Note the order: v1 - v0
    u.cross(v).normalize()
}

fn fill_triangle(a: Vec2, b: Vec2, c: Vec2, color: Color) {
    let a = macroquad::math::Vec2::new(a.x, a.y);
    let b = macroquad::math::Vec2::new(b.x, b.y);
    let c = macroquad::math::Vec2::new(c.x, c.y);
    draw_triangle(a, b, c, color);
}

#[macroquad::main("Renderer")]
async fn main() {
    let pipeline_params = PipelineParams {
        depth_write: true,
        depth_test: Comparison::LessOrEqual,
        ..Default::default()
    };

    let vertices = vec![
        vec3(-0.5, -0.5, -0.5), // 0
        vec3(0.5, -0.5, -0.5),  // 1
        vec3(0.5, 0.5, -0.5),   // 2
        vec3(-0.5, 0.5, -0.5),  // 3
        vec3(-0.5, -0.5, 0.5),  // 4
        vec3(0.5, -0.5, 0.5),   // 5
        vec3(0.5, 0.5, 0.5),    // 6
        vec3(-0.5, 0.5, 0.5),   // 7
    ];

    let indices = vec![
        0, 1, 2, 2, 3, 0, // front
        1, 5, 6, 6, 2, 1, // right
        7, 6, 5, 5, 4, 7, // back
        4, 0, 3, 3, 7, 4, // left
        4, 5, 1, 1, 0, 4, // bottom
        3, 2, 6, 6, 7, 3, // top
    ];

    let mut objects = vec![
        Object3D {
            vertices: vertices.clone(),
            indices: indices.clone(),

            position: vec3(0.0, 0.0, 0.0),
            rotation: vec3(0.0, 0.0, 0.0),
            scale: vec3(1.0, 1.0, 1.0),
        },
        Object3D {
            vertices: vertices.clone(),
            indices: indices.clone(),

            position: vec3(2.0, 0.0, 0.0),
            rotation: vec3(45.0, 45.0, 0.0),
            scale: vec3(1.0, 1.0, 1.0),
        },
        //Object3D::from_obj("/home/skynse/projects/tiny3d/assets/Car.obj"),
    ];

    let mut fov_y: f32 = 45.0;
    let mut aspect_ratio = screen_width() / screen_height();
    let mut z_near = 0.01;
    let mut z_far = 100.0;

    let mut yaw = -90.0;
    let mut pitch = 0.0;
    let mut camera_pos = vec3(0.0, 0.0, 5.0);
    let mut camera_target = vec3(0.0, 0.0, 0.0);
    let camera_up = vec3(0.0, 1.0, 0.0);
    let mut last_mouse_pos = mouse_position();
    let mut light_dir = vec3(0.0, 0.0, 1.0).normalize();
    loop {
        clear_background(BLACK);
        aspect_ratio = screen_width() / screen_height();
        let proj_mat = Mat4::perspective_rh(fov_y.to_radians(), aspect_ratio, z_near, z_far);
        let view_mat = Mat4::look_at_rh(camera_pos, camera_target, camera_up);

        for object in &mut objects {
            object.rotation.y += 0.5; // Rotate object
            draw_object(object, proj_mat, view_mat, camera_pos, light_dir);
        }

        draw_text(
            &format!("Camera Pos: {:?}", camera_pos),
            10.0,
            20.0,
            20.0,
            WHITE,
        );

        draw_text(&format!("Yaw: {:.2}", yaw), 10.0, 40.0, 20.0, WHITE);
        draw_text(&format!("Pitch: {:.2}", pitch), 10.0, 60.0, 20.0, WHITE);
        draw_text(&format!("Fov: {:.2}", fov_y), 10.0, 80.0, 20.0, WHITE);

        let (mouse_x, mouse_y) = mouse_wheel();
        if mouse_y > 0.0 {
            fov_y -= 10.0;
        } else if mouse_y < 0.0 {
            fov_y += 10.0;
        }

        if is_mouse_button_down(MouseButton::Left) {
            if is_key_down(KeyCode::LeftShift) {
                let (x, y) = mouse_position();
                let (last_x, last_y) = last_mouse_pos;
                let dx = x - last_x;
                let dy = y - last_y;

                let forward = (camera_target - camera_pos).normalize();
                let right = forward.cross(camera_up).normalize();
                let up = right.cross(forward).normalize();

                camera_pos -= right * dx * 0.01;
                camera_pos += up * dy * 0.01;
                camera_target -= right * dx * 0.01;
                camera_target += up * dy * 0.01;
            } else {
                let (x, y) = mouse_position();
                let (last_x, last_y) = last_mouse_pos;
                let dx = x - last_x;
                let dy = y - last_y;

                yaw -= dx * 0.1;
                pitch += dy * 0.1;

                pitch = pitch.clamp(-89.0, 89.0);

                let mut front = vec3(
                    pitch.to_radians().cos() * yaw.to_radians().cos(),
                    pitch.to_radians().sin(),
                    pitch.to_radians().cos() * yaw.to_radians().sin(),
                );

                front = front.normalize();

                camera_target = camera_pos + front;
            }
        }

        last_mouse_pos = mouse_position();

        next_frame().await
    }
}
