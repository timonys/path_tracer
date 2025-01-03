pub struct Sphere {
    center: Vec3,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) {
        Sphere { center, radius }
    }

    pub fn intersect(&self, ray_origin: Vec3, ray_dir: Vec3) -> Option<f32> {
        let oc = ray_origin - self.center;

        let a = 1.0f;
        let b = 2.0f * oc.dot(ray_dir);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = b * b - 4.0 * a * c;

        if (discriminant < 0.0) {
            None
        } else {
            let sqrt_discriminant = discriminant.sqrt();

            let t1 = (-b - sqrt_discriminant) / 2.0 * a;
            let t2 = (-b + sqrt_discriminant) / 2.0 * a;

            if (t1 > 0.0f) {
                Some(t1)
            } else if (t2 > 0.0f) {
                Some(t2)
            } else {
                None
            }
        }
    }
}
