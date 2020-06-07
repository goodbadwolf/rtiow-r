use std::ops::{Add, Div, Mul, Neg, Sub};

pub type Float = f64;

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    pub e: [Float; 3],
}

pub type Point = Vec3;
pub type Color = Vec3;

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Vec3 {
    pub fn new() -> Vec3 {
        Vec3 {
            e: [0 as Float, 0 as Float, 0 as Float],
        }
    }

    pub const fn with_elements(e0: Float, e1: Float, e2: Float) -> Vec3 {
        Vec3 { e: [e0, e1, e2] }
    }

    pub fn x(&self) -> Float {
        self.e[0]
    }

    pub fn y(&self) -> Float {
        self.e[1]
    }

    pub fn z(&self) -> Float {
        self.e[2]
    }

    pub fn length(&self) -> Float {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> Float {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::with_elements(
            self.e[0] + other.e[0],
            self.e[1] + other.e[1],
            self.e[2] + other.e[2],
        )
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self::with_elements(
            self.e[0] - other.e[0],
            self.e[1] - other.e[1],
            self.e[2] - other.e[2],
        )
    }
}

impl Mul<Float> for Vec3 {
    type Output = Self;

    fn mul(self, t: Float) -> Self::Output {
        Self::with_elements(self.e[0] * t, self.e[1] * t, self.e[2] * t)
    }
}

impl Div<Float> for Vec3 {
    type Output = Self;

    fn div(self, t: Float) -> Self::Output {
        self * (1 as Float / t)
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec3::with_elements(-self.e[0], -self.e[1], -self.e[2])
    }
}

impl Ray {
    pub fn new() -> Ray {
        Ray {
            origin: Vec3::new(),
            direction: Vec3::new(),
        }
    }

    pub fn with_data(origin: Vec3, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }

    pub fn at(self, t: Float) -> Point {
        self.origin + self.direction * t
    }
}

pub fn to_unit_vector(v: &Vec3) -> Vec3 {
    *v / v.length()
}

pub fn dot_product(u: &Vec3, v: &Vec3) -> Float {
    u.e[0] * v.e[0] + u.e[1] * v.e[1] + u.e[2] * v.e[2]
}

pub fn is_in_range(t: Float, t_min: Float, t_max: Float) -> bool {
    t < t_max && t > t_min
}
