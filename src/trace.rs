use crate::math::{
    clamp, dot_product, is_in_range, random_in_range, random_in_unit_hemisphere,
    reflect_around_normal, to_unit_vector, Color, Float, Point, Ray, Vec3,
};
use std::cmp::Ordering;
use std::f64::consts::PI;
use std::io::Write;
use std::rc::Rc;

pub const BLACK: Color = Color::with_elements(0.0 as Float, 0.0 as Float, 0.0 as Float);
pub const WHITE: Color = Color::with_elements(1.0 as Float, 1.0 as Float, 1.0 as Float);
const LIGHT_BLUE: Color = Color::with_elements(0.5 as Float, 0.7 as Float, 1.0 as Float);

pub struct HitRecord {
    pub point: Point,
    pub normal: Vec3,
    pub t: Float,
    pub front_face: bool,
    pub material: Rc<Material>,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: Float, t_max: Float) -> Option<HitRecord>;
}

pub struct Sphere {
    pub center: Point,
    pub radius: Float,
    pub material: Rc<Material>,
}

pub struct HittableCollection {
    pub hittables: Vec<Box<dyn Hittable>>,
}

pub struct Camera {
    origin: Point,
    lower_left_corner: Point,
    horizontal: Vec3,
    vertical: Vec3,
}

pub trait Material {
    fn scatter(&self, ray: &Ray, hit: &HitRecord, attenuation: &mut Color) -> Option<Ray>;
}

pub struct LambertianMaterial {
    pub albedo: Color,
}

pub struct MetalMaterial {
    pub albedo: Color,
}

impl HitRecord {
    pub fn from_hit(
        point: &Point,
        ray: &Ray,
        t: Float,
        outward_normal: &Vec3,
        material: Rc<Material>,
    ) -> Self {
        let mut result = HitRecord {
            point: *point,
            normal: Vec3::new(),
            t,
            front_face: false,
            material,
        };
        result.set_face_normal(ray, outward_normal);
        result
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        self.front_face = dot_product(&ray.direction, &outward_normal) < 0 as Float;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        };
    }
}

impl Sphere {
    pub fn new(center: &Point, radius: Float, material: Rc<Material>) -> Self {
        Sphere {
            center: *center,
            radius,
            material,
        }
    }

    fn calc_hit(&self, t: Float, ray: &Ray) -> HitRecord {
        let point = ray.at(t);
        let outward_normal = (point - self.center) / self.radius;
        HitRecord::from_hit(&point, &ray, t, &outward_normal, self.material.clone())
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = dot_product(&oc, &ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        match discriminant.partial_cmp(&(0 as Float)).unwrap() {
            Ordering::Less => None,
            _ => {
                let root = discriminant.sqrt();
                let t_root1 = (-half_b - root) / a;
                let t_root2 = (-half_b + root) / a;
                if is_in_range(t_root1, t_min, t_max) {
                    Some(self.calc_hit(t_root1, &ray))
                } else if is_in_range(t_root2, t_min, t_max) {
                    Some(self.calc_hit(t_root2, &ray))
                } else {
                    None
                }
            }
        }
    }
}

impl HittableCollection {
    pub fn new() -> Self {
        HittableCollection { hittables: vec![] }
    }

    pub fn with_hittable(hittable: Box<dyn Hittable>) -> Self {
        let mut result = Self::new();
        result.add(hittable);
        result
    }

    pub fn add(&mut self, hittable: Box<dyn Hittable>) {
        self.hittables.push(hittable);
    }
}

impl Hittable for HittableCollection {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_hit: Option<HitRecord> = None;
        let mut closest_hit_t = t_max;

        for hittable in self.hittables.iter() {
            if let Some(hit) = hittable.hit(&ray, t_min, closest_hit_t) {
                closest_hit_t = hit.t;
                closest_hit = Some(hit);
            }
        }

        closest_hit
    }
}

impl Camera {
    pub fn new(width: u32, height: u32) -> Self {
        let aspect_ratio = width as Float / height as Float;
        let viewport_height = 2 as Float;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1 as Float;

        let origin = Point::with_elements(0 as Float, 0 as Float, 1 as Float);
        let horizontal = Vec3::with_elements(viewport_width, 0 as Float, 0 as Float);
        let vertical = Vec3::with_elements(0 as Float, viewport_height, 0 as Float);
        let depth = Vec3::with_elements(0 as Float, 0 as Float, focal_length);
        let lower_left_corner = origin - horizontal / 2 as Float - vertical / 2 as Float - depth;

        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }

    pub fn get_ray(&self, u: Float, v: Float) -> Ray {
        let direction =
            self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin;
        Ray {
            origin: self.origin,
            direction,
        }
    }
}

impl Material for LambertianMaterial {
    fn scatter(&self, ray: &Ray, hit: &HitRecord, attenuation: &mut Color) -> Option<Ray> {
        let scatter_direction = hit.normal + lambertian_random_in_unit_sphere();
        *attenuation = self.albedo;
        let scattered_ray = Ray {
            origin: hit.point,
            direction: scatter_direction,
        };
        Some(scattered_ray)
    }
}

impl Material for MetalMaterial {
    fn scatter(&self, ray: &Ray, hit: &HitRecord, attenuation: &mut Color) -> Option<Ray> {
        let reflected_direction = reflect_around_normal(&ray.direction, &hit.normal);
        let scattered_ray = Ray {
            origin: hit.point,
            direction: reflected_direction,
        };
        *attenuation = self.albedo;
        if dot_product(&scattered_ray.direction, &hit.normal) > 0 as Float {
            Some(scattered_ray)
        } else {
            None
        }
    }
}

pub fn lambertian_random_in_unit_sphere() -> Vec3 {
    let a = random_in_range(0 as Float, 2 as Float * PI);
    let z = random_in_range(-1 as Float, 1 as Float);
    let r = (1 as Float - (z * z)).sqrt();
    Vec3::with_elements(r * a.cos(), r * a.sin(), z)
}

pub fn get_ray_color(ray: &Ray, world: &HittableCollection, depth: u32) -> Color {
    if depth == 0 {
        return BLACK;
    }

    if let Some(hit) = world.hit(ray, 0.001 as Float, Float::INFINITY) {
        let mut attenuation = WHITE;
        if let Some(scattered_ray) = hit.material.scatter(&ray, &hit, &mut attenuation) {
            return attenuation * get_ray_color(&scattered_ray, world, depth - 1);
        } else {
            return BLACK;
        }
    }

    let unit_direction = to_unit_vector(&ray.direction);
    let t = (unit_direction.y() + (1.0 as Float)) * (0.5 as Float);

    WHITE * (1.0 as Float - t) + LIGHT_BLUE * t
}

pub fn write_pixel(out: &mut dyn Write, pixel_color: &Color) -> std::io::Result<()> {
    let corrected_color = apply_gamma_correction(pixel_color);
    writeln!(
        out,
        "{} {} {}",
        to_color_byte(corrected_color.x()),
        to_color_byte(corrected_color.y()),
        to_color_byte(corrected_color.z())
    )
}

fn apply_gamma_correction(color: &Color) -> Color {
    Color::with_elements(color.e[0].sqrt(), color.e[1].sqrt(), color.e[2].sqrt())
}

fn to_color_byte(c: Float) -> u8 {
    ((256 as Float) * clamp(c, 0 as Float, 0.999 as Float)) as u8
}
