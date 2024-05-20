extern crate glam;
use crate::object::Object3D;
use crate::{generate_sphere, generate_torus, linalg::*};
use glam::{vec2, vec3, Mat4, Vec2, Vec3, Vec4, Vec4Swizzles};
use macroquad::prelude::*;

pub async fn run() {
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
            normals: vec![],
            position: vec3(0.0, 0.0, 0.0),
            rotation: vec3(0.0, 0.0, 0.0),
            scale: vec3(1.0, 1.0, 1.0),
        },
        //Object3D::from_obj("/home/skynse/projects/tiny3d/assets/Car.obj"),
    ];

    let mut fov_y: f32 = 45.0;
    let mut aspect_ratio = screen_width() / screen_height();
    let mut z_near = 0.01;
    let mut z_far = 100.0;

    let mut paused = true;

    let mut yaw = -90.0;
    let mut pitch = 0.0;
    let mut camera_pos = vec3(0.0, 0.0, 5.0);
    let mut camera_target = vec3(0.0, 0.0, 0.0);
    let camera_up = vec3(0.0, 1.0, 0.0);
    let mut last_mouse_pos = mouse_position();
    let mut light_dir = vec3(0.0, 0.0, 1.0).normalize();
    let mut iron_man = Object3D::from_obj(r"assets/Car.obj");

    objects.push(iron_man);

    let mut sphere = generate_sphere(1.0, 50, 50);
    sphere.position.x -= 5.0;
    objects.push(sphere);

    for object in &mut objects {
        compute_vertex_normals(object);
    }

    loop {
        clear_background(DARKGRAY);
        aspect_ratio = screen_width() / screen_height();
        let proj_mat = Mat4::perspective_rh(fov_y.to_radians(), aspect_ratio, z_near, z_far);
        let view_mat = Mat4::look_at_rh(camera_pos, camera_target, camera_up);

        for object in &mut objects {
            let dist: f32 = object.position.distance(camera_pos);
            if dist > 30.0 {
                continue;
            }
            object.rotation.y += 0.5 * paused as i32 as f32; // Rotate object

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

        fov_y -= mouse_y * 0.5;
        fov_y = fov_y.clamp(1.0, 90.0);

        if is_key_down(KeyCode::W) {
            camera_pos += (camera_target - camera_pos).normalize() * 0.1;
            camera_target += (camera_target - camera_pos).normalize() * 0.1;
        }

        if is_key_down(KeyCode::S) {
            camera_pos -= (camera_target - camera_pos).normalize() * 0.1;
            camera_target -= (camera_target - camera_pos).normalize() * 0.1;
        }

        if is_key_down(KeyCode::A) {
            let forward = (camera_target - camera_pos).normalize();
            let right = forward.cross(camera_up).normalize();
            camera_pos -= right * 0.1;
            camera_target -= right * 0.1;
        }

        if is_key_down(KeyCode::D) {
            let forward = (camera_target - camera_pos).normalize();
            let right = forward.cross(camera_up).normalize();
            camera_pos += right * 0.1;
            camera_target += right * 0.1;
        }

        if is_key_down(KeyCode::E) {
            camera_pos += camera_up * 0.1;
            camera_target += camera_up * 0.1;
        }

        if is_key_down(KeyCode::Q) {
            camera_pos -= camera_up * 0.1;
            camera_target -= camera_up * 0.1;
        }

        if is_key_pressed(KeyCode::Space) {
            paused = !paused;
        }

        // arrow keys for light direction

        if is_key_down(KeyCode::Up) {
            light_dir.z += 0.1;
        }

        if is_key_down(KeyCode::Down) {
            light_dir.z -= 0.1;
        }

        if is_key_down(KeyCode::Left) {
            light_dir.x -= 0.1;
        }

        if is_key_down(KeyCode::Right) {
            light_dir.x += 0.1;
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

                yaw += dx * 0.1;
                pitch -= dy * 0.1;

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

        // for each object, draw a gizmo that shows the orientation of the object

        last_mouse_pos = mouse_position();

        next_frame().await
    }
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
    let mut ndc_vertices: Vec<Vec2> = vec![]; // Normalized device coordinates

    // Phong shading parameters
    let ambient_strength = 0.1;
    let light_color = vec3(1.0, 1.5, 1.0);
    let object_color = vec3(1.0, 1.0, 1.0);
    let specular_strength = 0.5;
    let shininess = 32.0;

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
            let normal_view = (mv_mat
                * Vec4::new(normal_model.x, normal_model.y, normal_model.z, 0.0))
            .xyz()
            .normalize();

            // Backface culling
            if normal_view.z < 0.0 {
                // If the normal is pointing away from the camera
                continue;
            }

            // Phong shading calculations
            // Ambient
            let ambient = ambient_strength * light_color;

            // Diffuse
            let light_dir = light_dir.normalize();
            let diff = normal_view.dot(light_dir).max(0.0);
            let diffuse = diff * light_color;

            // Specular
            let view_dir = (camera_pos - object.position).normalize();
            let reflect_dir = reflect(-light_dir, normal_view);
            let spec = view_dir.dot(reflect_dir).max(0.0).powf(shininess);
            let specular = specular_strength * spec * light_color;

            // Combine results
            let result_color = (ambient + diffuse + specular) * object_color;
            let shaded_color = Color::new(result_color.x, result_color.y, result_color.z, 1.0);

            fill_triangle(screen_a, screen_b, screen_c, shaded_color);
            // Optionally draw wireframe
            // draw_wireframe_edges(screen_a, screen_b, screen_c);
        }
    }
}

// Utility function to reflect a vector
fn reflect(v: Vec3, normal: Vec3) -> Vec3 {
    v - 2.0 * v.dot(normal) * normal
}
