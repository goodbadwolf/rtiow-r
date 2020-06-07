use crate::math::{dot_product, is_in_range, to_unit_vector, Color, Float, Point, Ray, Vec3};
use std::cmp::Ordering;
use std::io::Write;
use std::rc::Rc;

const WHITE: Color = Color::with_elements(1.0 as Float, 1.0 as Float, 1.0 as Float);
const LIGHT_BLUE: Color = Color::with_elements(0.5 as Float, 0.7 as Float, 1.0 as Float);

#[derive(Debug, Copy, Clone)]
pub struct HitRecord {
    pub point: Point,
    pub normal: Vec3,
    pub t: Float,
    pub front_face: bool,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: Float, t_max: Float) -> Option<HitRecord>;
}

#[derive(Debug)]
pub struct Sphere {
    pub center: Point,
    pub radius: Float,
}

pub struct HittableCollection {
    pub hittables: Vec<Rc<dyn Hittable>>,
}

impl HitRecord {
    pub fn from_hit(point: &Point, ray: &Ray, t: Float, outward_normal: &Vec3) -> Self {
        let mut result = HitRecord {
            point: *point,
            normal: Vec3::new(),
            t,
            front_face: false,
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
    pub fn new(center: &Point, radius: Float) -> Self {
        Sphere {
            center: *center,
            radius,
        }
    }

    fn calc_hit(&self, t: Float, ray: &Ray) -> HitRecord {
        let point = ray.at(t);
        let outward_normal = (point - self.center) / self.radius;
        HitRecord::from_hit(&point, &ray, t, &outward_normal)
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

    pub fn with_hittable(hittable: Rc<dyn Hittable>) -> Self {
        let mut result = Self::new();
        result.add(hittable);
        result
    }

    pub fn add(&mut self, hittable: Rc<dyn Hittable>) {
        self.hittables.push(hittable);
    }
}

impl Hittable for HittableCollection {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_hit: Option<HitRecord> = None;
        let mut closest_hit_t = t_max;

        for hittable in self.hittables.iter() {
            if let Some(hit) = hittable.hit(&ray, t_min, closest_hit_t) {
                closest_hit = Some(hit);
                closest_hit_t = hit.t;
            }
        }

        closest_hit
    }
}

pub fn write_pixel(out: &mut dyn Write, pixel_color: &Vec3) -> std::io::Result<()> {
    writeln!(
        out,
        "{} {} {}",
        to_color_byte(pixel_color.x()),
        to_color_byte(pixel_color.y()),
        to_color_byte(pixel_color.z())
    )
}

pub fn get_ray_color(ray: &Ray, world: &HittableCollection) -> Color {
    if let Some(hit_normal) = world.hit(ray, 0 as Float, Float::INFINITY) {
        return (hit_normal.normal + WHITE) * 0.5 as Float;
    }

    let unit_direction = to_unit_vector(&ray.direction);
    let t = (unit_direction.y() + (1.0 as Float)) * (0.5 as Float);

    WHITE * (1.0 as Float - t) + LIGHT_BLUE * t
}

fn to_color_byte(c: Float) -> u8 {
    (c * (255.999 as Float)) as u8
}
