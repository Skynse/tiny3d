use std::{f32::consts::PI, vec};

use crate::glam::*;
use macroquad::prelude::*;
use std::thread;

fn rotateX(angle: f32) -> Mat4 {
    let angle = angle.to_radians();
    Mat4::from_cols(
        vec4(1.0, 0.0, 0.0, 0.0),
        vec4(0.0, angle.cos(), -angle.sin(), 0.0),
        vec4(0.0, angle.sin(), angle.cos(), 0.0),
        vec4(0.0, 0.0, 0.0, 1.0),
    )
}

fn rotateY(angle: f32) -> Mat4 {
    let angle = angle.to_radians();
    Mat4::from_cols(
        vec4(angle.cos(), 0.0, angle.sin(), 0.0),
        vec4(0.0, 1.0, 0.0, 0.0),
        vec4(-angle.sin(), 0.0, angle.cos(), 0.0),
        vec4(0.0, 0.0, 0.0, 1.0),
    )
}

fn rotateZ(angle: f32) -> Mat4 {
    let angle = angle.to_radians();
    Mat4::from_cols(
        vec4(angle.cos(), -angle.sin(), 0.0, 0.0),
        vec4(angle.sin(), angle.cos(), 0.0, 0.0),
        vec4(0.0, 0.0, 1.0, 0.0),
        vec4(0.0, 0.0, 0.0, 1.0),
    )
}

#[macroquad::main("Renderer")]
async fn main() {
    let mut angle = 0.0;
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

    let mut fov_y: f32 = 45.0;
    let mut aspect_ratio = screen_width() / screen_height();
    let mut z_near = 0.01;
    let mut z_far = 100.0;

    let proj_mat = Mat4::perspective_rh(fov_y.to_radians(), aspect_ratio, z_near, z_far);
    let mut yaw = 0.0;
    let mut pitch = 0.0;
    let mut camera_pos = vec3(0.0, 0.0, 5.0);
    let mut camera_target = vec3(0.0, 0.0, 0.0);
    let camera_up = vec3(0.0, 1.0, 0.0);
    let mut last_mouse_pos = mouse_position();

    let mut cursor_center = vec3(0.0, 0.0, 0.0);

    loop {
        clear_background(BLACK);
        aspect_ratio = screen_width() / screen_height();
        let proj_mat = Mat4::perspective_rh(fov_y.to_radians(), aspect_ratio, z_near, z_far);
        let view_mat = Mat4::look_at_rh(camera_pos, camera_target, camera_up);
        let model_mat = Mat4::IDENTITY;
        let mvp_mat = proj_mat * view_mat * model_mat;

        let mut cube_vertices = vec![];
        let rotation_matrix = rotateY(angle) * rotateX(angle);

        for vertex in vertices.iter() {
            let transformed_vertex = mvp_mat * Vec4::new(vertex.x, vertex.y, vertex.z, 1.0);

            cube_vertices.push(vec2(
                transformed_vertex.x / transformed_vertex.w,
                transformed_vertex.y / transformed_vertex.w,
            ));
        }

        for vertex in cube_vertices.iter() {
            draw_circle(
                vertex.x * screen_width() / 2.0 + screen_width() / 2.0,
                vertex.y * -screen_height() / 2.0 + screen_height() / 2.0,
                10.0,
                RED,
            );
        }

        let indices = vec![
            0, 1, 2, 2, 3, 0, // front
            1, 5, 6, 6, 2, 1, // right
            7, 6, 5, 5, 4, 7, // back
            4, 0, 3, 3, 7, 4, // left
            4, 5, 1, 1, 0, 4, // bottom
            3, 2, 6, 6, 7, 3, // top
        ];

        for i in 0..indices.len() / 3 {
            let i = i * 3;
            let a = cube_vertices[indices[i]];
            let b = cube_vertices[indices[i + 1]];
            let c = cube_vertices[indices[i + 2]];

            draw_line(
                a.x * screen_width() / 2.0 + screen_width() / 2.0,
                a.y * -screen_height() / 2.0 + screen_height() / 2.0,
                b.x * screen_width() / 2.0 + screen_width() / 2.0,
                b.y * -screen_height() / 2.0 + screen_height() / 2.0,
                2.0,
                WHITE,
            );
            draw_line(
                b.x * screen_width() / 2.0 + screen_width() / 2.0,
                b.y * -screen_height() / 2.0 + screen_height() / 2.0,
                c.x * screen_width() / 2.0 + screen_width() / 2.0,
                c.y * -screen_height() / 2.0 + screen_height() / 2.0,
                2.0,
                WHITE,
            );
            draw_line(
                c.x * screen_width() / 2.0 + screen_width() / 2.0,
                c.y * -screen_height() / 2.0 + screen_height() / 2.0,
                a.x * screen_width() / 2.0 + screen_width() / 2.0,
                a.y * -screen_height() / 2.0 + screen_height() / 2.0,
                2.0,
                WHITE,
            );
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

        angle += 0.5;
        angle %= 360.0;

        let (mouse_x, mouse_y) = mouse_wheel();
        if mouse_y > 0.0 {
            fov_y -= 10.0;
        } else if mouse_y < 0.0 {
            fov_y += 10.0;
        }

        if is_mouse_button_down(MouseButton::Middle) {
            if is_key_down(KeyCode::LeftShift) {
                let (x, y) = mouse_position();
                let (last_x, last_y) = last_mouse_pos;
                let dx = x - last_x;
                let dy = y - last_y;

                let forward = (camera_target - camera_pos).normalize();
                let right = forward.cross(camera_up).normalize();
                let up = right.cross(forward).normalize();

                camera_pos -= right * dx * 0.01;
                camera_pos -= up * dy * 0.01;
                camera_target -= right * dx * 0.01;
                camera_target -= up * dy * 0.01;
            } else {
                let (x, y) = mouse_position();
                let (last_x, last_y) = last_mouse_pos;
                let dx = x - last_x;
                let dy = y - last_y;

                yaw -= dx * 0.1;
                pitch += dy * 0.1;

                if pitch > 89.0 {
                    pitch = 89.0;
                }
                if pitch < -89.0 {
                    pitch = -89.0;
                }

                let mut front = vec3(
                    pitch.to_radians().cos() * yaw.to_radians().cos(),
                    pitch.to_radians().sin(),
                    pitch.to_radians().cos() * yaw.to_radians().sin(),
                );

                front = front.normalize();

                camera_target = camera_pos + front;
            }
        }

        if is_key_down(KeyCode::Left) {
            cursor_center.x -= 0.1;
        }
        if is_key_down(KeyCode::Right) {
            cursor_center.x += 0.1;
        }
        if is_key_down(KeyCode::Up) {
            cursor_center.y += 0.1;
        }
        if is_key_down(KeyCode::Down) {
            cursor_center.y -= 0.1;
        }

        last_mouse_pos = mouse_position();

        let cursor_center =
            mvp_mat * Vec4::new(cursor_center.x, cursor_center.y, cursor_center.z, 1.0);

        draw_circle_lines(
            cursor_center.x / cursor_center.w * screen_width() / 2.0 + screen_width() / 2.0,
            cursor_center.y / cursor_center.w * -screen_height() / 2.0 + screen_height() / 2.0,
            10.0,
            2.0,
            GREEN,
        );

        draw_cube(
            vec3(0.0, 0.0, 0.0),
            vec3(1.0, 1.0, 1.0),
            None,
            Color::new(1.0, 0.0, 0.0, 1.0),
        );

        next_frame().await
    }
}
