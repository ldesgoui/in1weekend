use crate::prelude::*;

const ANTI_ACNE: Scalar = 0.001;

pub trait Material {
    fn scatter(&self, _ray: &Ray, _intersection: &RayIntersection) -> Option<(Ray, Color)> {
        None
    }

    fn emitted(&self, _ray: &Ray, _intersection: &RayIntersection) -> Color {
        Color::default()
    }
}

pub struct Lambertian<T: Texture> {
    pub albedo: T,
}

impl<T: Texture> Material for Lambertian<T> {
    fn scatter(&self, ray: &Ray, intersection: &RayIntersection) -> Option<(Ray, Color)> {
        let target = intersection.point(&ray) + intersection.normal + Vector::random_in_sphere();

        Some((
            Ray {
                origin: intersection.point(&ray) + intersection.normal * ANTI_ACNE,
                dir: target - intersection.point(&ray),
            },
            self.albedo.sample(&ray, &intersection),
        ))
    }
}

pub struct Metal<T: Texture> {
    pub albedo: T,
    pub fuzz: Scalar,
}

impl<T: Texture> Material for Metal<T> {
    fn scatter(&self, ray: &Ray, intersection: &RayIntersection) -> Option<(Ray, Color)> {
        let reflected = ray.dir.normalize().reflect(&intersection.normal);

        if reflected.dot(&intersection.normal) <= 0. {
            return None;
        }

        Some((
            Ray {
                origin: intersection.point(&ray) + intersection.normal * ANTI_ACNE,
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
    fn scatter(&self, ray: &Ray, intersection: &RayIntersection) -> Option<(Ray, Color)> {
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
                        origin: intersection.point(&ray) - intersection.normal * ANTI_ACNE,
                        dir: refracted,
                    },
                    self.attenuation.sample(&ray, &intersection),
                ));
            }
        }

        Some((
            Ray {
                origin: intersection.point(&ray) + intersection.normal * ANTI_ACNE,
                dir: ray.dir.reflect(&intersection.normal),
            },
            self.attenuation.sample(&ray, &intersection),
        ))
    }
}

pub struct DiffuseLight<T: Texture> {
    pub value: T,
}

impl<T: Texture> Material for DiffuseLight<T> {
    fn scatter(&self, _: &Ray, _: &RayIntersection) -> Option<(Ray, Color)> {
        None
    }

    fn emitted(&self, ray: &Ray, intersection: &RayIntersection) -> Color {
        self.value.sample(ray, intersection)
    }
}

pub struct Isotropic<T: Texture> {
    pub albedo: T,
}

impl<T: Texture> Material for Isotropic<T> {
    fn scatter(&self, ray: &Ray, intersection: &RayIntersection) -> Option<(Ray, Color)> {
        Some((
            Ray {
                origin: intersection.point(&ray) + intersection.normal * ANTI_ACNE,
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
