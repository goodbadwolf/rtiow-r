use rand::Rng;
use std::f64::consts::PI;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub};

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
    pub const fn new(e0: Float, e1: Float, e2: Float) -> Vec3 {
        Vec3 { e: [e0, e1, e2] }
    }

    pub fn random() -> Vec3 {
        Vec3::new(random_float(), random_float(), random_float())
    }

    pub fn random_in_range(min: Float, max: Float) -> Vec3 {
        Vec3::new(
            random_in_range(min, max),
            random_in_range(min, max),
            random_in_range(min, max),
        )
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

    fn add(self, rhs: Self) -> Self {
        Self::new(
            self.e[0] + rhs.e[0],
            self.e[1] + rhs.e[1],
            self.e[2] + rhs.e[2],
        )
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.e[0] += rhs.e[0];
        self.e[1] += rhs.e[1];
        self.e[2] += rhs.e[2];
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(
            self.e[0] - rhs.e[0],
            self.e[1] - rhs.e[1],
            self.e[2] - rhs.e[2],
        )
    }
}

impl Mul<Float> for Vec3 {
    type Output = Self;

    fn mul(self, t: Float) -> Self::Output {
        Self::new(self.e[0] * t, self.e[1] * t, self.e[2] * t)
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec3::new(
            self.e[0] * rhs.e[0],
            self.e[1] * rhs.e[1],
            self.e[2] * rhs.e[2],
        )
    }
}

impl MulAssign<Float> for Vec3 {
    fn mul_assign(&mut self, t: Float) {
        self.e[0] *= t;
        self.e[1] *= t;
        self.e[2] *= t;
    }
}

impl Div<Float> for Vec3 {
    type Output = Self;

    fn div(self, t: Float) -> Self::Output {
        self * (1 as Float / t)
    }
}

impl DivAssign<Float> for Vec3 {
    fn div_assign(&mut self, t: Float) {
        *self *= 1 as Float / t;
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec3::new(-self.e[0], -self.e[1], -self.e[2])
    }
}

impl Ray {
    pub fn new() -> Ray {
        Ray {
            origin: Vec3::new(0 as Float, 0 as Float, 0 as Float),
            direction: Vec3::new(0 as Float, 0 as Float, 0 as Float),
        }
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

pub fn cross_product(u: &Vec3, v: &Vec3) -> Vec3 {
    Vec3::new(
        u.e[1] * v.e[2] - u.e[2] * v.e[1],
        u.e[2] * v.e[0] - u.e[0] * v.e[2],
        u.e[0] * v.e[1] - u.e[1] * v.e[0],
    )
}

pub fn is_in_range(t: Float, t_min: Float, t_max: Float) -> bool {
    t < t_max && t > t_min
}

pub fn clamp(x: Float, min: Float, max: Float) -> Float {
    if x < min {
        return min;
    } else if x > max {
        return max;
    }
    x
}

pub fn random_float() -> Float {
    rand::thread_rng().gen()
}

pub fn random_in_range(min: Float, max: Float) -> Float {
    rand::thread_rng().gen_range(min, max)
}

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let point = Vec3::random_in_range(-1 as Float, 1 as Float);
        if point.length_squared() > 1 as Float {
            continue;
        } else {
            return point;
        }
    }
}

pub fn random_in_unit_disk() -> Vec3 {
    loop {
        let p = Vec3::new(
            random_in_range(-1 as Float, 1 as Float),
            random_in_range(-1 as Float, 1 as Float),
            0 as Float,
        );
        if p.length_squared() >= 1 as Float {
            continue;
        } else {
            return p;
        }
    }
}

pub fn random_in_unit_hemisphere(normal: &Vec3) -> Vec3 {
    let in_unit_sphere = random_in_unit_sphere();
    if dot_product(&in_unit_sphere, normal) > 0 as Float {
        in_unit_sphere
    } else {
        -in_unit_sphere
    }
}

pub fn reflect_around_normal(v: &Vec3, normal: &Vec3) -> Vec3 {
    *v - *normal * 2 as Float * dot_product(v, normal)
}

pub fn refract_around_normal(u: &Vec3, normal: &Vec3, etai_over_etat: Float) -> Vec3 {
    let cos_theta = dot_product(&(-*u), normal);
    let r_out_parallel = (*u + (*normal * cos_theta)) * etai_over_etat;
    let r_out_perp = *normal * -(1.0 as Float - r_out_parallel.length_squared()).sqrt();
    return r_out_parallel + r_out_perp;
}

pub fn degrees_to_radians(theta: Float) -> Float {
    (theta * PI) / 180 as Float
}
