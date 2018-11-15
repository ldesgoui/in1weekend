#![feature(duration_as_u128)]

extern crate image;
extern crate log;
extern crate nalgebra as na;
extern crate ncollide3d as nc;
extern crate palette;
extern crate pretty_env_logger;
extern crate rand;

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() -> Result<(), failure::Error> {
    pretty_env_logger::init();
    info!("version: {}", VERSION);

    info!("creating scene");
    let scene = Scene::default();

    info!("grabbing camera");
    let camera = Camera::default();

    info!("checking it");
    camera.bench(&scene, 10000);

    info!("capturing");
    let picture = camera.capture(&scene);

    info!("saving");
    picture.save("out.png")?;

    info!("viewing result");
    std::process::Command::new("feh")
        .args(&["-F", "out.png"])
        .status()?;

    warn!("success, exiting");
    Ok(())
}

type AABB = nc::bounding_volume::AABB<Scalar>;
type BVT = nc::partitioning::BVT<Box<ObjectTrait>, AABB>;
type Isometry = na::Isometry3<Scalar>;
type Point = na::Point3<Scalar>;
type Ray = nc::query::Ray<Scalar>;
type Scalar = f32;
type RayIntersection = nc::query::RayIntersection<Scalar>;
type Vector = na::Vector3<Scalar>;
use palette::LinSrgb;
type RayCast = nc::query::RayCast<Scalar>;

trait ObjectTrait {
    fn raycast(&self, ray: &Ray) -> Option<RayIntersection>;
    fn material_scatter(&self, ray: &Ray, intersection: &RayIntersection)
        -> Option<(Ray, LinSrgb)>;

    fn material_emitted_using_intersection(
        &self,
        ray: &Ray,
        intersection: &RayIntersection,
    ) -> LinSrgb;
}
impl<M: Material, S: nc::query::RayCast<Scalar>> ObjectTrait for Object<M, S> {
    fn raycast(&self, ray: &Ray) -> Option<RayIntersection> {
        self.shape
            .toi_and_normal_and_uv_with_ray(&self.transform, ray, true)
    }

    fn material_scatter(
        &self,
        ray: &Ray,
        intersection: &RayIntersection,
    ) -> Option<(Ray, LinSrgb)> {
        self.material.scatter(&ray, &intersection)
    }
    fn material_emitted_using_intersection(
        &self,
        ray: &Ray,
        intersection: &RayIntersection,
    ) -> LinSrgb {
        self.material
            .emitted_using_intersection(&ray, &intersection)
    }
}

#[derive(Debug)]
struct Camera {
    origin: Point,
    top_left_corner: Point,
    horizontal: Vector,
    vertical: Vector,
    u: Vector,
    v: Vector,

    lens_radius: Scalar,
    shutter_speed: Scalar,

    resolution: na::Vector2<u32>,
    samples: u32,
}

impl Default for Camera {
    fn default() -> Self {
        Self::new(
            &Point::origin(),
            &Point::new(0.0, 0.0, -1.0),
            &Vector::y().into(),
            90.0,
            0.0,
            None,
            1.0 / 500.0,
            na::Vector2::new(1000, 1000),
            20,
        )
    }
}

impl Camera {
    fn new(
        from: &Point,
        at: &Point,
        up: &Vector,
        vfov: Scalar,
        aperture: Scalar,
        focus_dist: Option<Scalar>,
        shutter_speed: Scalar,
        resolution: na::Vector2<u32>,
        samples: u32,
    ) -> Self {
        let theta = vfov.to_radians();
        let half_height = (theta / 2.0).tan();
        let half_width = (resolution.x as f32 / resolution.y as f32) * half_height;
        let w = (from - at).normalize();
        let u = up.cross(&w).normalize();
        let v = w.cross(&u);
        let focus_dist = focus_dist.unwrap_or((at - from).magnitude());
        Self {
            origin: *from,
            top_left_corner: from - half_width * focus_dist * u + half_height * focus_dist * v
                - focus_dist * w,
            horizontal: 2.0 * half_width * focus_dist * u,
            vertical: 2.0 * half_height * focus_dist * v,
            u: u,
            v: v,
            lens_radius: aperture / 2.0,
            shutter_speed: shutter_speed,
            resolution: resolution,
            samples: samples,
        }
    }

    fn bench(&self, scene: &Scene, pixels: u32) {
        use rand::Rng;

        debug!("benchmarking {:?} random pixels", pixels);

        let before_render = std::time::Instant::now();
        let mut rng = rand::thread_rng();
        let mut img = image::RgbImage::new(1, 1);

        for _ in 0..pixels {
            let color = self.capture_pixel(
                scene,
                rng.gen_range(0, self.resolution.x),
                rng.gen_range(0, self.resolution.y),
            );
            img.put_pixel(0, 0, color);
        }

        debug!(
            "results: {:?} ({:?} p/s)",
            before_render.elapsed(),
            pixels as f32 / (before_render.elapsed().as_nanos() as f32 / 1_000_000_000.0)
        );
        debug!(
            "estimating for full render: {:?}",
            (before_render.elapsed() / pixels) * self.resolution.x * self.resolution.y
        );
    }

    fn capture(&self, scene: &Scene) -> image::RgbImage {
        // TODO: parallelism
        // TODO: generate chunks and stitch chunks
        // (I assume this helps data locality? generating a
        // big image with low sample amount is much slower than
        // generating a small image with large sample amount)
        let before_render = std::time::Instant::now();
        let mut last_print = std::time::Instant::now();
        let second = std::time::Duration::from_secs(1);
        let total_pixels = self.resolution.x * self.resolution.y;

        let ret = image::RgbImage::from_fn(self.resolution.x, self.resolution.y, |x, y| {
            if last_print.elapsed() > second {
                let pixels = y * self.resolution.x + x;
                debug!(
                    "progress: {:5.1}% in {:-3.1?}",
                    100.0 * pixels as f32 / total_pixels as f32,
                    elapsed = before_render.elapsed(),
                );
                last_print = std::time::Instant::now();
            }
            self.capture_pixel(scene, x, y)
        });
        debug!(
            "total render: {:?} ({:?} p/s)",
            before_render.elapsed(),
            total_pixels as f32 / (before_render.elapsed().as_nanos() as f32 / 1_000_000_000.0)
        );
        ret
    }

    fn capture_pixel(&self, scene: &Scene, x: u32, y: u32) -> image::Rgb<u8> {
        // TODO: shutter speed / motion blur
        // TODO: extract sample distribution ?

        let color = (0..self.samples).fold(LinSrgb::default(), |color, _| {
            let u = (rand::random::<f32>() + x as f32) / self.resolution.x as f32;
            let v = (rand::random::<f32>() + y as f32) / self.resolution.y as f32;
            color + scene.trace(&self.ray(u, v), 0)
        }) / self.samples as f32;

        let srgb: palette::Srgb<u8> = palette::Srgb::from_linear(color).into_format();

        use palette::Pixel;
        image::Rgb {
            data: *srgb.as_raw(),
        }
    }

    fn ray(&self, u: f32, v: f32) -> Ray {
        let rd = self.lens_radius * rand_in_disk();
        let offset = self.u * rd.x + self.v * rd.y;

        Ray {
            origin: self.origin + offset,
            dir: self.top_left_corner + u * self.horizontal
                - v * self.vertical
                - self.origin
                - offset,
        }
    }
}

struct Scene {
    objects: BVT,
    background: palette::gradient::Gradient<LinSrgb>,
    max_depth: u32,
}

impl Scene {
    fn trace(&self, ray: &Ray, depth: u32) -> LinSrgb {
        match self
            .objects
            .best_first_search(&mut (CostByRayCast { ray: &ray }))
        {
            None => self.background.get((ray.dir.normalize().y + 1.0) / 2.0),
            Some((object, intersection)) => {
                let emitted = object.material_emitted_using_intersection(&ray, &intersection);

                if depth > self.max_depth {
                    return emitted;
                }
                if let Some((scattered, attenuation)) = object.material_scatter(&ray, &intersection)
                {
                    attenuation * self.trace(&scattered, depth + 1)
                } else {
                    emitted
                }
            }
        }
    }

    fn new(objects: Vec<(Box<ObjectTrait>, AABB)>) -> Scene {
        Scene {
            objects: BVT::new_balanced(objects),
            background: palette::gradient::Gradient::new(vec![
                LinSrgb::new(0.4, 0.5, 1.0),
                LinSrgb::new(1.0, 1.0, 1.0),
                LinSrgb::new(0.4, 0.5, 1.0),
            ]),
            max_depth: 10,
        }
    }
}

impl Default for Scene {
    fn default() -> Self {
        use ncollide3d::bounding_volume::HasBoundingVolume;

        let transform = Isometry::new(Vector::new(1.0, 0.0, -1.0), Vector::zeros());
        let transform1 = Isometry::new(Vector::new(0.0, 0.0, -1.0), Vector::zeros());
        let transform2 = Isometry::new(Vector::new(-1.0, 0.0, -1.0), Vector::zeros());
        let transform3 = Isometry::new(Vector::new(0.0, -100.5, 0.0), Vector::zeros());
        let transform4 = Isometry::new(Vector::new(0.0, -0.5, -1.0), Vector::zeros());

        Self::new(vec![
            (
                Box::new(Object {
                    material: Lambertian {
                        albedo: Checkerboard {
                            odd: UVTexture,
                            even: PointTexture,
                            size: 10.0,
                        },
                    },
                    shape: nc::shape::Ball::new(0.5),
                    transform: transform,
                }),
                nc::shape::Ball::new(0.5).bounding_volume(&transform),
            ),
            (
                Box::new(Object {
                    material: Dielectric { refraction: 5.0 },
                    shape: nc::shape::Ball::new(0.5),
                    transform: transform1,
                }),
                nc::shape::Ball::new(0.5).bounding_volume(&transform1),
            ),
            (
                Box::new(Object {
                    material: Isotropic {
                        albedo: LinSrgb::new(0.3, 0.5, 0.8),
                    },
                    shape: ConstantMedium {
                        shape: nc::shape::Ball::new(0.5),
                        density: 0.5,
                    },
                    transform: transform2,
                }),
                nc::shape::Ball::new(0.5).bounding_volume(&transform2),
            ),
            (
                Box::new(Object {
                    material: Metal {
                        albedo: Checkerboard {
                            odd: LinSrgb::new(0.0, 0.0, 0.0),
                            even: LinSrgb::new(1.0, 1.0, 1.0),
                            size: 10.0,
                        },
                        fuzz: 0.0,
                    },
                    shape: nc::shape::Ball::new(100.0),
                    transform: transform3,
                }),
                nc::shape::Ball::new(100.0).bounding_volume(&transform3),
            ),
            (
                Box::new(Object {
                    material: DiffuseLight {
                        value: LinSrgb::new(1.0, 1.0, 1.0),
                    },
                    shape: nc::shape::Ball::new(0.5),
                    transform: transform4,
                }),
                nc::shape::Ball::new(0.5).bounding_volume(&transform4),
            ),
        ])
    }
}

struct CostByRayCast<'a> {
    ray: &'a Ray,
}

impl<'a> nc::partitioning::BVTCostFn<Scalar, Box<ObjectTrait>, AABB> for CostByRayCast<'a> {
    type UserData = RayIntersection;

    fn compute_bv_cost(&mut self, bv: &AABB) -> Option<Scalar> {
        use ncollide3d::query::RayCast;

        bv.toi_with_ray(&Isometry::identity(), self.ray, true)
    }

    fn compute_b_cost(&mut self, b: &Box<ObjectTrait>) -> Option<(Scalar, Self::UserData)> {
        b.raycast(self.ray).map(|i| (i.toi, i))
    }
}

struct Object<M: Material, S: nc::query::RayCast<Scalar>> {
    material: M,
    shape: S,
    transform: Isometry,
}

struct ConstantMedium<S: nc::query::RayCast<Scalar>> {
    shape: S,
    density: Scalar,
}

impl<S: nc::query::RayCast<Scalar>> nc::query::RayCast<Scalar> for ConstantMedium<S> {
    fn toi_and_normal_with_ray(
        &self,
        m: &Isometry,
        ray: &Ray,
        solid: bool,
    ) -> Option<RayIntersection> {
        let intersection1 = self.shape.toi_and_normal_with_ray(m, ray, solid)?;
        let new_ray = Ray {
            origin: intersection1.point(&ray) + (ray.dir * 0.0001),
            dir: ray.dir,
        };
        let intersection2 = self.shape.toi_and_normal_with_ray(m, ray, solid)?;
        let distance_through =
            (intersection2.point(&new_ray) - intersection1.point(&ray)).magnitude();
        let hit_distance = -(1.0 / self.density) * rand::random::<Scalar>().ln();
        if hit_distance >= distance_through {
            return None;
        }
        Some(RayIntersection {
            toi: intersection1.toi + hit_distance,
            normal: Vector::y(),
            uvs: None,
        })
    }
}

trait Material {
    fn scatter(&self, ray: &Ray, intersection: &RayIntersection) -> Option<(Ray, LinSrgb)>;
    fn emitted(&self, _: Scalar, _: Scalar, _: Point) -> LinSrgb {
        Default::default()
    }

    fn emitted_using_intersection(&self, ray: &Ray, intersection: &RayIntersection) -> LinSrgb {
        if let Some(uvs) = intersection.uvs {
            self.emitted(uvs.x, uvs.y, intersection.point(&ray))
        } else {
            Default::default()
        }
    }
}

struct Lambertian<T: Texture> {
    albedo: T,
}

impl<T: Texture> Material for Lambertian<T> {
    fn scatter(&self, ray: &Ray, intersection: &RayIntersection) -> Option<(Ray, LinSrgb)> {
        let target = intersection.point(&ray) + intersection.normal + rand_in_sphere();

        Some((
            Ray {
                origin: intersection.point(&ray),
                dir: target - intersection.point(&ray),
            },
            self.albedo.sample(0.0, 0.0, intersection.point(&ray)),
        ))
    }
}

struct Metal<T: Texture> {
    albedo: T,
    fuzz: Scalar,
}

impl<T: Texture> Material for Metal<T> {
    fn scatter(&self, ray: &Ray, intersection: &RayIntersection) -> Option<(Ray, LinSrgb)> {
        let reflected = reflect(&ray.dir.normalize(), &intersection.normal);

        if reflected.dot(&intersection.normal) <= 0.0 {
            return None;
        }

        Some((
            Ray {
                origin: intersection.point(&ray),
                dir: reflected + self.fuzz * rand_in_sphere(),
            },
            self.albedo.sample(0.0, 0.0, intersection.point(&ray)),
        ))
    }
}

struct Dielectric {
    refraction: Scalar,
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, intersection: &RayIntersection) -> Option<(Ray, LinSrgb)> {
        let attenuation = LinSrgb::new(1.0, 1.0, 1.0);
        let rdotn = ray.dir.dot(&intersection.normal);
        let (outward_normal, ni_over_nt, cosine) = if rdotn > 0.0 {
            let cosine = rdotn / ray.dir.magnitude();
            let cosine = (1.0 - self.refraction * self.refraction * (1.0 - cosine * cosine)).sqrt();
            (-intersection.normal, self.refraction, cosine)
        } else {
            let cosine = -rdotn / ray.dir.magnitude();
            (intersection.normal, 1.0 / self.refraction, cosine)
        };
        if let Some(refracted) = refract(&ray.dir, &outward_normal, ni_over_nt) {
            let reflect_prob = schlick(cosine, self.refraction);
            if rand::random::<f32>() > reflect_prob {
                return Some((
                    Ray {
                        origin: intersection.point(&ray),
                        dir: refracted,
                    },
                    attenuation,
                ));
            }
        }
        Some((
            Ray {
                origin: intersection.point(&ray),
                dir: reflect(&ray.dir, &intersection.normal),
            },
            attenuation,
        ))
    }
}

struct DiffuseLight<T: Texture> {
    value: T,
}

impl<T: Texture> Material for DiffuseLight<T> {
    fn scatter(&self, _: &Ray, _: &RayIntersection) -> Option<(Ray, LinSrgb)> {
        None
    }

    fn emitted(&self, u: Scalar, v: Scalar, p: Point) -> LinSrgb {
        self.value.sample(u, v, p)
    }
}

struct Isotropic<T: Texture> {
    albedo: T,
}

impl<T: Texture> Material for Isotropic<T> {
    fn scatter(&self, ray: &Ray, intersection: &RayIntersection) -> Option<(Ray, LinSrgb)> {
        Some((
            Ray {
                origin: intersection.point(&ray),
                dir: rand_in_sphere(),
            },
            self.albedo.sample(
                intersection.uvs?.x,
                intersection.uvs?.y,
                intersection.point(&ray),
            ),
        ))
    }
}

trait Texture {
    fn sample(&self, u: Scalar, v: Scalar, p: Point) -> LinSrgb;
}

impl Texture for LinSrgb {
    fn sample(&self, _: Scalar, _: Scalar, _: Point) -> LinSrgb {
        *self
    }
}

struct Checkerboard<E: Texture, O: Texture> {
    even: E,
    odd: O,
    size: Scalar,
}

impl<E: Texture, O: Texture> Texture for Checkerboard<E, O> {
    fn sample(&self, u: Scalar, v: Scalar, p: Point) -> LinSrgb {
        let p = p * self.size;
        if p.x.sin() * p.y.sin() * p.z.sin() < 0.0 {
            self.odd.sample(u, v, p)
        } else {
            self.even.sample(u, v, p)
        }
    }
}

struct UVTexture;

impl Texture for UVTexture {
    fn sample(&self, u: Scalar, v: Scalar, _p: Point) -> LinSrgb {
        LinSrgb::new(u, v, 0.0)
    }
}

struct PointTexture;

impl Texture for PointTexture {
    fn sample(&self, _u: Scalar, _v: Scalar, p: Point) -> LinSrgb {
        LinSrgb::new(p.x, p.y, p.z)
    }
}

// UTILS

fn rand_in_disk() -> Point {
    let theta: Scalar = 2.0 * std::f32::consts::PI * rand::random::<Scalar>();

    Point::new(theta.cos(), theta.sin(), 0.0)
}

fn rand_in_sphere() -> Vector {
    rand::random::<Vector>().normalize() * rand::random::<Scalar>().cbrt()
}

fn reflect(v: &Vector, n: &Vector) -> Vector {
    v - 2.0 * v.dot(&n) * n
}

fn refract(v: &Vector, n: &Vector, ni_over_nt: Scalar) -> Option<Vector> {
    let uv = v.normalize();
    let dt = uv.dot(&n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if discriminant > 0.0 {
        Some(ni_over_nt * (uv - n * dt) - n * discriminant.sqrt())
    } else {
        None
    }
}

fn schlick(cosine: Scalar, refraction: Scalar) -> Scalar {
    let r0 = (1.0 - refraction) / (1.0 + refraction);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
}

trait IntersectionPoint {
    fn point(&self, ray: &Ray) -> Point;
}

impl IntersectionPoint for RayIntersection {
    fn point(&self, ray: &Ray) -> Point {
        ray.origin + ray.dir * self.toi
    }
}
