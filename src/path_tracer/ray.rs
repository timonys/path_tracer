pub struct Ray{
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3)
    {
        Ray {origin, direction}
    }
}