use crate::background::create_background_color;
use crate::path_tracer::materials::*;
use crate::path_tracer::path_trace_components::*;
use crate::path_tracer::path_tracer_structs::*;
use crate::utils::*;
use flecs_ecs::prelude::*;
use glam::Vec3;

pub fn generate_rays(
    cam: &CameraComponent,
    path_tracer: &PathTracerComponent,
    ray_buffer: &mut RayBufferComponent,
) {
    for y in 0..path_tracer.height {
        for x in 0..path_tracer.width {
            for _s in 0..path_tracer.sample_amount {
                let ray = create_ray(cam, x as f32, y as f32);
                ray_buffer.rays.push(ray);
                ray_buffer.indices.push((x, y));
            }
        }
    }
}

pub fn trace_color(
    entity: EntityView,
    path_tracer: &PathTracerComponent,
    ray_buffer: &RayBufferComponent,
    sample_buffer: &mut AccumulatedSampleBufferComponent,
) {
    for (i, ray) in ray_buffer.rays.iter().enumerate() {
        let (x, y) = ray_buffer.indices[i];
        let mut color = get_color(entity, ray, path_tracer.max_depth);

        let sample_index = y as usize * sample_buffer.width + x as usize;
        gamma_correct_color(&mut color);

        // Accumulate the color for the pixel
        sample_buffer.sample_data[sample_index] += color / path_tracer.sample_amount as f32;
    }
}

pub fn create_ray(cam: &CameraComponent, u_coord: f32, v_coord: f32) -> Ray {
    //pixel sample with random offset for anti aliasing
    let sample_offset = rand_vec3();
    let pixel_sample = cam.pixel_upper_left_pos
        + ((u_coord + sample_offset.x) * cam.pixel_delta_u)
        + ((v_coord + sample_offset.y) * cam.pixel_delta_v);
    let ray_direction = pixel_sample - cam.pos;

    Ray::new(cam.pos, ray_direction.normalize())
}

pub fn trace(entity: EntityView, ray: &Ray, ray_min: f32, ray_max: f32) -> Option<Hit> {
    let query = entity
        .world()
        .new_query::<(&ShapeComponent, &MaterialComponent, &TransformComponent)>();
    let mut closest_intersection: Option<Hit> = None;
    let mut current_ray_max = ray_max;

    query.each(|(shape, mat, transform)| {
        if let Some(hit) =
            shape
                .shape
                .intersect(&transform.pos, ray, &mat.material, ray_min, ray_max)
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

    create_background_color(ray)
}
