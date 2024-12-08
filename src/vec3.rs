use std::{io::{BufWriter, Write}, ops::{Add, Div, Mul, Sub}};

use rand::{random, thread_rng, Rng};

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
    fn gamma_correct(x: f64) -> f64 {
        f64::sqrt(x)
    }
    pub fn write_as_color<W>(&self, out: &mut BufWriter<W>) 
        where W: std::io::Write
    {
        out.write_all(format!("{} {} {}\n", 
            (Self::gamma_correct(f64::clamp(self.x, 0.0, 1.0)) * 255.0) as i32, 
            (Self::gamma_correct(f64::clamp(self.y, 0.0, 1.0)) * 255.0) as i32, 
            (Self::gamma_correct(f64::clamp(self.z, 0.0, 1.0)) * 255.0) as i32).as_bytes())
        .unwrap();
    }
    pub fn zero() -> Self {
        Vec3{ x: 0.0, y: 0.0, z: 0.0}
    }
    pub fn normalized(&self) -> Self {
        let inv_len = 1.0 / self.len();
        inv_len * self
    }
    pub fn negate(&mut self) {
        self.x = -self.x;
        self.y = -self.y;
        self.z = -self.z;
    }
    pub fn negated(&self) -> Vec3 {
        let mut other = self.clone();
        other.negate();
        other
    }
    pub fn random(min: f64, max: f64) -> Vec3 {
        let mut rng = thread_rng();
        Vec3 { x: rng.gen_range(min..max), y: rng.gen_range(min..max), z: rng.gen_range(min..max) }
    }
    pub fn random_unit_vec() -> Vec3 {
        let mut vec = Self::random(-1.0, 1.0);
        let mut lsq = vec.sq_len();
        //we dont below 10^-160 to avoid f64::INFINITY after normalization
        while lsq > 1.0 || lsq < 1e-160 {
            vec = Self::random(-1.0, 1.0);
            lsq = vec.sq_len()
        }
        vec.normalized()
    }
    pub fn random_unit_vec_on_hemisphere(vec: &Vec3) -> Vec3 {
        let mut random = Self::random_unit_vec();
        if Self::dot(&random, vec) <= 0.0 {
            random.negate();
        }
        random
    }
    pub fn is_near_zero(&self) -> bool {
        f64::abs(self.x) < 1e-8
            && f64::abs(self.y) < 1e-8
            && f64::abs(self.z) < 1e-8
    }
    pub fn reflect(&self, n: &Vec3) -> Vec3 {
        self + &(-2.0 * Self::dot(self, n) * n)
    }
    //I also don't understnad how this works
    pub fn refract(&self, n: &Vec3, idx: f64) -> Vec3 {
        let cos_theta = Self::dot(&self.negated(), n).min(1.0);
        let out_x = (self + &(cos_theta * n)) * idx;
        let out_y = -(1.0 - out_x.sq_len()).abs().sqrt() * n;
        out_x + out_y
    }
}
impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3{ x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
    }
}
impl Add for &Vec3 {
    type Output = Vec3;

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
impl Sub for &Vec3 {
    type Output = Vec3;

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
impl Mul<&Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: &Vec3) -> Self::Output {
        Vec3{ x: rhs.x * self, y: rhs.y * self, z: rhs.z * self }
    }
}
impl From<(f64, f64, f64)> for Vec3 {
    fn from(value: (f64, f64, f64)) -> Self {
        Vec3{ x: value.0, y: value.1, z: value.2 }
    }
}