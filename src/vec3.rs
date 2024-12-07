use std::{io::{BufWriter, Write}, ops::{Add, Div, Mul, Sub}};

#[derive(Clone)]
pub struct Vec3 {
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
    pub fn write_as_color<W>(&self, out: &mut BufWriter<W>) 
        where W: std::io::Write
    {
        out.write_all(format!("{} {} {}\n", (self.x * 256.0) as i32, (self.y * 256.0) as i32, (self.z * 256.0) as i32).as_bytes())
        .unwrap();
    }
}
impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3{ x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
    }
}
impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3{ x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z }
    }
}
impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec3{ x: self.x * rhs.x, y: self.y * rhs.y, z: self.z * rhs.z }
    }
}
impl Div for Vec3 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Vec3{ x: self.x / rhs.x, y: self.y / rhs.y, z: self.z / rhs.z }
    }
}
impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3{ x: self.x * rhs, y: self.y * rhs, z: self.z * rhs }
    }
}
impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3{ x: rhs.x * self, y: rhs.y * self, z: rhs.z * self }
    }
}