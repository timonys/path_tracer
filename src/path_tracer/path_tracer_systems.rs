use crate::path_tracer::path_trace_components::*;
use crate::path_tracer::path_tracer_structs::*;
use crate::utils::*;
use crate::CameraComponent;
use flecs_ecs::prelude::*;
use glam::Vec3;

pub fn path_trace(
    entity: EntityView,
    path_tracer: &mut PathTracerComponent,
    sample_buffer: &mut AccumulatedSampleBufferComponent,
) {
    let camera_query = entity.world().new_query::<&CameraComponent>();
    let width = sample_buffer.width;
    let height = sample_buffer.height;

    camera_query.each(|cam| {
        for y in 0..height {
            for x in 0..width {
                let mut color = Vec3::ZERO;
                for s in 0..path_tracer.sample_amount {
                    let ray = create_ray(cam, x as f32, y as f32);

                    //Trace sample
                    color += get_color(entity, &ray, path_tracer.max_depth);
                }

                let sample_index = y * width + x;
                let pixel_samples_scale = 1.0 / path_tracer.sample_amount as f32;
                color *= pixel_samples_scale;
                gamma_correct_color(&mut color);
                sample_buffer.sample_data[sample_index] = color;
            }
        }
    });
    path_tracer.has_rendered = true;
}

pub fn create_ray(cam: &CameraComponent, u_coord: f32, v_coord: f32) -> Ray {
    let focal_length = 1.0;
    let cam_height = 2.0;
    let cam_width = cam_height * (cam.viewport_width as f32 / cam.viewport_height as f32);

    // Calculate the vectors across the horizontal and down the vertical viewport edges.
    let viewport_u = Vec3::new(cam_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -cam_height, 0.0);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    let pixel_delta_u = viewport_u / cam.viewport_width as f32;
    let pixel_delta_v = viewport_v / cam.viewport_height as f32;

    // Calculate the location of the upper left pixel.
    let viewport_upper_left =
        cam.pos - Vec3::new(0.0, 0.0, focal_length) - viewport_u * 0.5 - viewport_v * 0.5;
    let pixel_upper_left_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    //pixel sample with random offset for anti aliasing
    let sample_offset = rand_vec3();
    let pixel_sample = pixel_upper_left_loc
        + ((u_coord + sample_offset.x) * pixel_delta_u)
        + ((v_coord + sample_offset.y) * pixel_delta_v);
    let ray_direction = pixel_sample - cam.pos;

    Ray {
        origin: cam.pos,
        dir: ray_direction,
    }
}

pub fn trace(entity: EntityView, ray: &Ray, ray_min: f32, ray_max: f32) -> Option<Hit> {
    let query = entity
        .world()
        .new_query::<(&ShapeComponent, &MaterialComponent)>();
    let mut closest_intersection: Option<Hit> = None;
    let mut current_ray_max = ray_max;

    query.each(|(shape_comp, mat_comp)| {
        if let Some(hit) =
            shape_comp
                .shape
                .intersect(ray, mat_comp.material.clone(), ray_min, ray_max)
        {
            if closest_intersection.is_none() || hit.t < closest_intersection.as_ref().unwrap().t {
                closest_intersection = Some(hit);
                current_ray_max = closest_intersection.as_ref().unwrap().t;
            }
        }
    });
    closest_intersection
}

pub fn get_color(entity: EntityView, ray: &Ray, depth: i32) -> Vec3 {
    // Base case for recursion: if depth reaches 0, return black
    if depth <= 0 {
        return Vec3::new(0.0, 0.0, 0.0);
    }

    // Trace the ray and check if it hits an object
    if let Some(hit) = trace(entity, ray, 0.001, f32::INFINITY) {
        let mut color_attenuation = Vec3::ZERO; // Initialize color attenuation
        let mut scattered_ray = Ray {
            origin: Vec3::ZERO,
            dir: Vec3::ZERO,
        };

        // If the ray scatters, apply the attenuation recursively and return the result
        if hit
            .material
            .scatter(ray, &hit, &mut color_attenuation, &mut scattered_ray)
        {
            // Apply attenuation to the recursively traced scattered ray
            let color = get_color(entity, &scattered_ray, depth - 1);
            color_attenuation *= color;
            return color_attenuation; // Return the computed color attenuation
        }
        return Vec3::ZERO;
    }

    // Background color (sky color) if no hit or scattering is unsuccessful
    let unit_direction = ray.dir.normalize();
    let a = 0.5 * (unit_direction.y + 1.0); // Smooth transition between sky and ground
    (1.0 - a) * Vec3::new(1.0, 1.0, 1.0) + a * Vec3::new(0.5, 0.7, 1.0)
}
