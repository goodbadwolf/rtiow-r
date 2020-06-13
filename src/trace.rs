use crate::math::{
    clamp, cross_product, degrees_to_radians, dot_product, is_in_range, random_float,
    random_in_range, random_in_unit_disk, random_in_unit_hemisphere, reflect_around_normal,
    refract_around_normal, to_unit_vector, Color, Float, Point, Ray, Vec3,
};
use std::cmp::Ordering;
use std::f64::consts::PI;
use std::io::Write;
use std::rc::Rc;

pub const BLACK: Color = Color::new(0.0 as Float, 0.0 as Float, 0.0 as Float);
pub const WHITE: Color = Color::new(1.0 as Float, 1.0 as Float, 1.0 as Float);
const LIGHT_BLUE: Color = Color::new(0.5 as Float, 0.7 as Float, 1.0 as Float);

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
    u: Vec3,
    v: Vec3,
    w: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Point,
    lens_radius: Float,
}

pub trait Material {
    fn scatter(&self, ray: &Ray, hit: &HitRecord, attenuation: &mut Color) -> Option<Ray>;
}

pub struct LambertianMaterial {
    pub albedo: Color,
}

pub struct MetalMaterial {
    pub albedo: Color,
    pub fuzziness: Float,
}

pub struct DiaelectriMaterial {
    pub albedo: Color,
    pub ref_idx: Float,
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
            normal: Vec3::new(0 as Float, 0 as Float, 0 as Float),
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

        match discriminant.partial_cmp(&(0 as Float)) {
            Some(Ordering::Less) => None,
            None => None,
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
    pub fn new(
        look_from: &Point,
        look_at: &Point,
        vup: &Vec3,
        vfov: Float,
        aspect_ratio: Float,
        aperture: Float,
        focus_distance: Float,
    ) -> Self {
        let theta = degrees_to_radians(vfov);
        let h = (theta / 2 as Float).tan();
        let viewport_height = 2 as Float * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = to_unit_vector(&(*look_from - *look_at));
        let u = to_unit_vector(&cross_product(&vup, &w));
        let v = cross_product(&w, &u);

        let origin = *look_from;
        let horizontal = u * viewport_width * focus_distance;
        let vertical = v * viewport_height * focus_distance;
        let lower_left_corner =
            origin - horizontal / 2 as Float - vertical / 2 as Float - w * focus_distance;

        Camera {
            origin,
            u,
            v,
            w,
            horizontal,
            vertical,
            lower_left_corner,
            lens_radius: aperture / 2 as Float,
        }
    }

    pub fn get_ray(&self, s: Float, t: Float) -> Ray {
        let rd = random_in_unit_disk() * self.lens_radius;
        let offset = self.u * rd.x() + self.v * rd.y();
        let origin = self.origin + offset;
        let direction = self.lower_left_corner + self.horizontal * s + self.vertical * t - origin;
        Ray { origin, direction }
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
        let fuzzed_direction =
            reflected_direction + lambertian_random_in_unit_sphere() * self.fuzziness;
        let scattered_ray = Ray {
            origin: hit.point,
            direction: fuzzed_direction,
        };
        *attenuation = self.albedo;
        if dot_product(&scattered_ray.direction, &hit.normal) > 0 as Float {
            Some(scattered_ray)
        } else {
            None
        }
    }
}

impl DiaelectriMaterial {
    pub fn new(ref_idx: Float) -> Self {
        DiaelectriMaterial {
            ref_idx,
            albedo: WHITE,
        }
    }

    fn schlick(cosine: Float, ref_idx: Float) -> Float {
        let r0 = (1 as Float - ref_idx) / (1 as Float + ref_idx);
        let r0 = r0 * r0;
        r0 + (1 as Float - r0) * (1 as Float - cosine).powi(5)
    }
}

impl Material for DiaelectriMaterial {
    fn scatter(&self, ray: &Ray, hit: &HitRecord, attenuation: &mut Color) -> Option<Ray> {
        *attenuation = self.albedo;
        let etai_over_etat = if hit.front_face {
            1 as Float / self.ref_idx
        } else {
            self.ref_idx
        };
        let direction = to_unit_vector(&ray.direction);
        let cos_thetha = dot_product(&(-direction), &hit.normal).min(1 as Float);
        let sin_thetha = (1 as Float - cos_thetha * cos_thetha).sqrt();
        let reflect_prob = DiaelectriMaterial::schlick(cos_thetha, etai_over_etat);
        let scattered_direction =
            if (etai_over_etat * sin_thetha > 1 as Float) || reflect_prob > random_float() {
                reflect_around_normal(&direction, &hit.normal)
            } else {
                refract_around_normal(&direction, &hit.normal, etai_over_etat)
            };

        let scattered_ray = Ray {
            origin: hit.point,
            direction: scattered_direction,
        };
        Some(scattered_ray)
    }
}

pub fn lambertian_random_in_unit_sphere() -> Vec3 {
    let a = random_in_range(0 as Float, 2 as Float * PI);
    let z = random_in_range(-1 as Float, 1 as Float);
    let r = (1 as Float - (z * z)).sqrt();
    Vec3::new(r * a.cos(), r * a.sin(), z)
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
    Color::new(color.e[0].sqrt(), color.e[1].sqrt(), color.e[2].sqrt())
}

fn to_color_byte(c: Float) -> u8 {
    ((256 as Float) * clamp(c, 0 as Float, 0.999 as Float)) as u8
}
