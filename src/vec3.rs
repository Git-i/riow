#[derive(Clone)]
struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}
impl Vec3 {
    pub fn sq_len(&self) -> f64 {
        Self::dot(self, self)
    }
    pub fn len(&self) -> f64 {
        self.sq_len().sqrt()
    }
    pub fn dot(lhs: &Self, rhs: &Self) -> f64 {
        lhs.x * rhs.x + lhs.y * rhs.y + lhs.z * rhs.z
    }
}