use crate::prelude::*;

pub trait Material {
    fn scatter(
        &self,
        _ray: &Ray,
        _intersection: &RayIntersection,
        _importance_sample: &Option<(Vector, Scalar)>,
    ) -> Option<(Ray, Color)> {
        None
    }

    fn emitted(&self, _ray: &Ray, _intersection: &RayIntersection) -> Color {
        Color::default()
    }

    fn important(&self) -> bool {
        false
    }
}

pub struct Lambertian<T: Texture> {
    pub albedo: T,
}

impl<T: Texture> Material for Lambertian<T> {
    fn scatter(
        &self,
        ray: &Ray,
        intersection: &RayIntersection,
        importance_sample: &Option<(Vector, Scalar)>,
    ) -> Option<(Ray, Color)> {
        let (direction, pdf_value) = if let Some((important_direction, important_pdf)) =
            importance_sample
        {
            let direction = if rand::random() {
                *important_direction
            } else {
                let rotation = na::Rotation3::rotation_between(&Vector::z(), &intersection.normal)
                    .unwrap_or(na::Rotation3::new(Vector::x() * std::f32::consts::PI));
                rotation * random_cosine_direction()
            };
            let cosine_pdf = intersection.normal.dot(&direction).max(0.) / std::f32::consts::PI;
            (direction, cosine_pdf * ((important_pdf + cosine_pdf) / 2.))
        } else {
            let rotation = na::Rotation3::rotation_between(&Vector::z(), &intersection.normal)
                .unwrap_or(na::Rotation3::new(Vector::x() * std::f32::consts::PI));
            let direction = rotation * random_cosine_direction();
            let cosine_pdf = intersection.normal.dot(&direction).max(0.) / std::f32::consts::PI;
            (direction, cosine_pdf * cosine_pdf)
        };

        Some((
            Ray {
                origin: intersection.point_nudged_out(&ray),
                dir: direction,
            },
            self.albedo.sample(&ray, &intersection) * pdf_value,
        ))
    }
}

fn random_cosine_direction() -> Vector {
    let r1: Scalar = rand::random();
    let r2: Scalar = rand::random();
    let z = (1. - r2).sqrt();
    let phi = 2. * std::f32::consts::PI * r1;
    let (s, c) = phi.sin_cos();
    let r2 = r2.sqrt();
    let x = c * 2. * r2;
    let y = s * 2. * r2;
    Vector::new(x, y, z).normalize()
}

pub struct Metal<T: Texture> {
    pub albedo: T,
    pub fuzz: Scalar,
}

impl<T: Texture> Material for Metal<T> {
    fn scatter(
        &self,
        ray: &Ray,
        intersection: &RayIntersection,
        _importance_sample: &Option<(Vector, Scalar)>,
    ) -> Option<(Ray, Color)> {
        let reflected = ray.dir.normalize().reflect(&intersection.normal);

        if reflected.dot(&intersection.normal) <= 0. {
            return None;
        }

        Some((
            Ray {
                origin: intersection.point_nudged_out(&ray),
                dir: reflected + self.fuzz * Vector::random_in_sphere(),
            },
            self.albedo.sample(&ray, &intersection),
        ))
    }
}

pub struct Dielectric<T: Texture> {
    pub refraction: Scalar,
    pub attenuation: T,
}

impl<T: Texture> Material for Dielectric<T> {
    fn scatter(
        &self,
        ray: &Ray,
        intersection: &RayIntersection,
        _importance_sample: &Option<(Vector, Scalar)>,
    ) -> Option<(Ray, Color)> {
        let rdotn = ray.dir.dot(&intersection.normal);

        let (outward_normal, ni_over_nt, cosine) = if rdotn > 0. {
            let cosine = self.refraction * rdotn / ray.dir.magnitude();
            (-intersection.normal, self.refraction, cosine)
        } else {
            let cosine = -rdotn / ray.dir.magnitude();
            (intersection.normal, 1. / self.refraction, cosine)
        };

        if let Some(refracted) = refract(&ray.dir, &outward_normal, ni_over_nt) {
            let reflect_prob = schlick(cosine, self.refraction);
            if rand::random::<f32>() > reflect_prob {
                return Some((
                    Ray {
                        origin: intersection.point_nudged_in(&ray),
                        dir: refracted,
                    },
                    self.attenuation.sample(&ray, &intersection),
                ));
            }
        }

        Some((
            Ray {
                origin: intersection.point_nudged_out(&ray),
                dir: ray.dir.reflect(&intersection.normal),
            },
            self.attenuation.sample(&ray, &intersection),
        ))
    }

    fn important(&self) -> bool {
        true
    }
}

pub struct DiffuseLight<T: Texture> {
    pub value: T,
}

impl<T: Texture> Material for DiffuseLight<T> {
    fn emitted(&self, ray: &Ray, intersection: &RayIntersection) -> Color {
        self.value.sample(ray, intersection)
    }

    fn important(&self) -> bool {
        true
    }
}

pub struct Isotropic<T: Texture> {
    pub albedo: T,
}

impl<T: Texture> Material for Isotropic<T> {
    fn scatter(
        &self,
        ray: &Ray,
        intersection: &RayIntersection,
        _importance_sample: &Option<(Vector, Scalar)>,
    ) -> Option<(Ray, Color)> {
        Some((
            Ray {
                origin: intersection.point_nudged_out(&ray),
                dir: Vector::random_on_sphere(),
            },
            self.albedo.sample(&ray, &intersection),
        ))
    }
}

pub fn refract(v: &Vector, n: &Vector, ni_over_nt: Scalar) -> Option<Vector> {
    let uv = v.normalize();
    let dt = uv.dot(&n);
    let discriminant = 1. - ni_over_nt * ni_over_nt * (1. - dt * dt);
    if discriminant > 0. {
        Some(ni_over_nt * (uv - n * dt) - n * discriminant.sqrt())
    } else {
        None
    }
}

pub fn schlick(cosine: Scalar, refraction: Scalar) -> Scalar {
    let r0 = (1. - refraction) / (1. + refraction);
    let r0 = r0 * r0;
    r0 + (1. - r0) * (1. - cosine).powi(5)
}
