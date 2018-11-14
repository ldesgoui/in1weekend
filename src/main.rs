#![feature(range_contains)]

extern crate image;
extern crate log;
extern crate nalgebra as na;
// extern crate ncollide3d as nc;
extern crate palette;
extern crate pretty_env_logger;
extern crate rand;

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() -> Result<(), failure::Error> {
    pretty_env_logger::init();

    let width = 1000;
    let height = 500;
    let samples = 50;

    info!("version: {}", VERSION);

    let look_from = vec3(-2.0, 2.0, 1.0);
    let look_at = vec3(0.0, 0.0, -1.0);
    let camera = Camera::new(
        &look_from,
        &look_at,
        &vec3(0.0, 1.0, 0.0),
        50.0,
        width as f32 / height as f32,
        0.0,
        (look_from - look_at).magnitude(),
        0.0,
        0.1,
    );
    let world: Vec<Box<Hitable>> = vec![
        Box::new(Sphere {
            center: vec3(0.0, 0.0, -1.0),
            radius: 0.25,
            material: Material::Dielectric { refraction: 4.0 },
            linear_velocity: Vec3::zeros(),
        }),
        Box::new(Sphere {
            center: vec3(1.0, 0.0, -1.0),
            radius: 0.5,
            material: Material::Metal {
                albedo: palette::Srgb::new(0.8, 0.6, 0.2).into_linear(),
                fuzz: 0.0,
            },
            linear_velocity: Vec3::zeros(),
        }),
        Box::new(Sphere {
            center: vec3(-1.0, 0.0, -1.0),
            radius: 0.5,
            material: Material::Metal {
                albedo: palette::Srgb::new(0.6, 0.6, 0.6).into_linear(),
                fuzz: 1.0,
            },
            linear_velocity: Vec3::zeros(),
        }),
        Box::new(Sphere {
            center: vec3(0.0, 0.0, -4.0),
            radius: 1.0,
            material: Material::Lambertian {
                albedo: palette::Srgb::new(0.1, 0.2, 0.5).into_linear(),
            },
            linear_velocity: vec3(0.0, 0.5, 0.0),
        }),
        Box::new(Sphere {
            center: vec3(0.0, -100.5, -1.0),
            radius: 100.0,
            material: Material::Lambertian {
                albedo: palette::Srgb::new(0.8, 0.8, 0.0).into_linear(),
            },
            linear_velocity: Vec3::zeros(),
        }),
    ];

    let before_render = std::time::Instant::now();

    info!(
        "rendering an image of dimensions {} by {} pixels with {}x sampling",
        width, height, samples
    );
    let buf = image::RgbImage::from_fn(width, height, |x, y| {
        use std::ops::Div;

        if x == 0 {
            info!(
                "{:5.1}% complete after {:?}",
                100.0 * y as f32 / height as f32,
                before_render.elapsed()
            );
        }

        let srgb: palette::Srgb<u8> = palette::Srgb::from_linear(
            (0..samples)
                .fold(palette::Srgb::new(0.0, 0.0, 0.0).into_linear(), |acc, _| {
                    let u = (rand::random::<f32>() + x as f32) / width as f32;
                    let v = (rand::random::<f32>() + y as f32) / height as f32;
                    acc + color(&world, &camera.ray(u, v), 0)
                })
                .div(samples as f32),
        )
        .into_format();

        use palette::Pixel;
        image::Rgb {
            data: *srgb.as_raw(),
        }
    });

    info!("render complete, run time: {:?}", before_render.elapsed());

    info!("writing to out.png");
    buf.save("out.png")?;

    info!("displaying result");
    std::process::Command::new("feh")
        .args(&["-F", "out.png"])
        .status()?;

    warn!("success, exiting");
    Ok(())
}

fn color(world: &Vec<Box<Hitable>>, ray: &Ray, depth: u32) -> palette::LinSrgb {
    match world.hit(&ray, 0.001, std::f32::INFINITY) {
        Some(hit) => {
            if depth > 50 {
                return palette::LinSrgb::default();
            }
            match hit.material.scatter(&ray, &hit) {
                Some((scattered, attenuation)) => {
                    return attenuation * color(&world, &scattered, depth + 1);
                }
                _ => {
                    return palette::LinSrgb::default();
                }
            }
        }
        _ => (),
    }

    use palette::Mix;
    let t = 0.5 * (ray.direction.normalize().y + 1.0);
    palette::Srgb::new(1.0, 1.0, 1.0)
        .into_linear()
        .mix(&palette::Srgb::new(0.5, 0.7, 1.0).into_linear(), t)
}

// RAY

struct Ray {
    origin: Vec3,
    direction: Vec3,
    time: f32,
}

impl Ray {
    fn point_at_parameter(&self, t: f32) -> Vec3 {
        self.origin + t * self.direction
    }
}

// HIT

struct Hit {
    t: f32,
    point: Vec3,
    normal: Vec3,
    material: Material,
}

trait Hitable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit>;
}

impl Hitable for Vec<Box<Hitable>> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        trace!("ray traversing world");
        let mut result = None;
        let mut closest_so_far = t_max;
        for hitable in self {
            match hitable.hit(ray, t_min, closest_so_far) {
                Some(hit) => {
                    closest_so_far = hit.t;
                    result = Some(hit);
                }
                _ => {}
            }
        }
        result
    }
}

// SPHERE

struct Sphere {
    center: Vec3,
    radius: f32,
    material: Material,
    linear_velocity: Vec3,
}

impl Sphere {
    fn center_at(&self, time: f32) -> Vec3 {
        self.center + self.linear_velocity * time
    }
}

impl Hitable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let oc = ray.origin - self.center_at(ray.time);
        let a = ray.direction.dot(&ray.direction);
        let b = oc.dot(&ray.direction);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;

        if discriminant > 0.0 {
            let t = (-b - discriminant.sqrt()) / a;
            if (t_min..t_max).contains(&t) {
                let point = ray.point_at_parameter(t);
                return Some(Hit {
                    t: t,
                    point: point,
                    normal: (point - self.center) / self.radius,
                    material: self.material,
                });
            }

            let t = (-b + discriminant.sqrt()) / a;
            if (t_min..t_max).contains(&t) {
                let point = ray.point_at_parameter(t);
                return Some(Hit {
                    t: t,
                    point: point,
                    normal: (point - self.center) / self.radius,
                    material: self.material,
                });
            }
        }
        None
    }
}

// CAMERA

struct Camera {
    top_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    origin: Vec3,
    u: Vec3,
    v: Vec3,
    lens_radius: f32,
    t0: f32,
    t1: f32,
}

impl Camera {
    fn new(
        from: &Vec3,
        at: &Vec3,
        up: &Vec3,
        vfov: f32,
        aspect: f32,
        aperture: f32,
        focus_dist: f32,
        t0: f32,
        t1: f32,
    ) -> Self {
        let theta = vfov.to_radians();
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        let w = (from - at).normalize();
        let u = up.cross(&w).normalize();
        let v = w.cross(&u);
        Self {
            top_left_corner: from - half_width * focus_dist * u + half_height * focus_dist * v
                - focus_dist * w,
            horizontal: 2.0 * half_width * focus_dist * u,
            vertical: 2.0 * half_height * focus_dist * v,
            origin: *from,
            u: u,
            v: v,
            lens_radius: aperture / 2.0,
            t0: t0,
            t1: t1,
        }
    }

    fn ray(&self, u: f32, v: f32) -> Ray {
        use rand::Rng;

        let rd = self.lens_radius * rand_in_disk();
        let offset = self.u * rd.x + self.v * rd.y;
        Ray {
            origin: self.origin + offset,
            direction: self.top_left_corner + u * self.horizontal
                - v * self.vertical
                - self.origin
                - offset,
            time: rand::thread_rng().gen_range(self.t0, self.t1),
        }
    }
}
// MATERIAL

#[derive(Clone, Copy)]
enum Material {
    Lambertian { albedo: palette::LinSrgb },
    Metal { albedo: palette::LinSrgb, fuzz: f32 },
    Dielectric { refraction: f32 },
}

impl Material {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<(Ray, palette::LinSrgb)> {
        match self {
            &Material::Lambertian { albedo } => {
                let target = hit.point + hit.normal + rand_in_sphere();

                Some((
                    Ray {
                        origin: hit.point,
                        direction: target - hit.point,
                        time: ray.time,
                    },
                    albedo,
                ))
            }
            &Material::Metal { albedo, fuzz } => {
                let reflected = reflect(ray.direction.normalize(), hit.normal);

                if reflected.dot(&hit.normal) <= 0.0 {
                    return None;
                }

                Some((
                    Ray {
                        origin: hit.point,
                        direction: reflected + fuzz * rand_in_sphere(),
                        time: ray.time,
                    },
                    albedo,
                ))
            }
            &Material::Dielectric { refraction } => {
                let attenuation = palette::Srgb::new(1.0, 1.0, 1.0).into_linear();
                let rdotn = ray.direction.dot(&hit.normal);
                let (outward_normal, ni_over_nt, cosine) = if rdotn > 0.0 {
                    let cosine = rdotn / ray.direction.magnitude();
                    let cosine = (1.0 - refraction * refraction * (1.0 - cosine * cosine)).sqrt();
                    (-hit.normal, refraction, cosine)
                } else {
                    let cosine = -rdotn / ray.direction.magnitude();
                    (hit.normal, 1.0 / refraction, cosine)
                };
                if let Some(refracted) = refract(&ray.direction, &outward_normal, ni_over_nt) {
                    let reflect_prob = schlick(cosine, refraction);
                    if rand::random::<f32>() > reflect_prob {
                        return Some((
                            Ray {
                                origin: hit.point,
                                direction: refracted,
                                time: ray.time,
                            },
                            attenuation,
                        ));
                    }
                }
                Some((
                    Ray {
                        origin: hit.point,
                        direction: reflect(ray.direction, hit.normal),
                        time: ray.time,
                    },
                    attenuation,
                ))
            }
        }
    }
}

// UTILS

type Vec3 = na::Vector3<f32>;

fn vec3(x: f32, y: f32, z: f32) -> Vec3 {
    Vec3::new(x, y, z)
}

fn rand_in_disk() -> Vec3 {
    loop {
        let point = 2.0 * vec3(rand::random(), rand::random(), 0.0) - vec3(1.0, 1.0, 0.0);
        if point.dot(&point) < 1.0 {
            return point;
        }
    }
}

fn rand_in_sphere() -> Vec3 {
    loop {
        let point =
            2.0 * vec3(rand::random(), rand::random(), rand::random()) - vec3(1.0, 1.0, 1.0);
        if point.magnitude_squared() < 1.0 {
            return point;
        }
    }
}

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * v.dot(&n) * n
}

fn refract(v: &Vec3, n: &Vec3, ni_over_nt: f32) -> Option<Vec3> {
    let uv = v.normalize();
    let dt = uv.dot(&n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if discriminant > 0.0 {
        Some(ni_over_nt * (uv - n * dt) - n * discriminant.sqrt())
    } else {
        None
    }
}

fn schlick(cosine: f32, refraction: f32) -> f32 {
    let r0 = (1.0 - refraction) / (1.0 + refraction);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
}
