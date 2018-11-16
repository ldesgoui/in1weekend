use crate::prelude::*;

const ANTI_ACNE: Scalar = 0.001;

pub trait Material {
    fn scatter(&self, ray: &Ray, intersection: &RayIntersection) -> Option<(Ray, LinSrgb)>;

    fn emitted(&self, _: Option<na::Point2<Scalar>>, _: Point) -> LinSrgb {
        LinSrgb::default()
    }

    fn emitted_using_intersection(&self, ray: &Ray, intersection: &RayIntersection) -> LinSrgb {
        self.emitted(intersection.uvs, intersection.point(&ray))
    }
}

pub struct Lambertian<T: Texture> {
    pub albedo: T,
}

impl<T: Texture> Material for Lambertian<T> {
    fn scatter(&self, ray: &Ray, intersection: &RayIntersection) -> Option<(Ray, LinSrgb)> {
        let target = intersection.point(&ray) + intersection.normal + rand_in_sphere();

        Some((
            Ray {
                origin: intersection.point(&ray) + intersection.normal * ANTI_ACNE,
                dir: target - intersection.point(&ray),
            },
            self.albedo.sample_using_intersection(&ray, &intersection),
        ))
    }
}

pub struct Metal<T: Texture> {
    pub albedo: T,
    pub fuzz: Scalar,
}

impl<T: Texture> Material for Metal<T> {
    fn scatter(&self, ray: &Ray, intersection: &RayIntersection) -> Option<(Ray, LinSrgb)> {
        let reflected = ray.dir.normalize().reflect(&intersection.normal);

        if reflected.dot(&intersection.normal) <= 0.0 {
            return None;
        }

        Some((
            Ray {
                origin: intersection.point(&ray) + intersection.normal * ANTI_ACNE,
                dir: reflected + self.fuzz * rand_in_sphere(),
            },
            self.albedo.sample_using_intersection(&ray, &intersection),
        ))
    }
}

pub struct Dielectric {
    pub refraction: Scalar,
    pub attenuation: LinSrgb,
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, intersection: &RayIntersection) -> Option<(Ray, LinSrgb)> {
        let rdotn = ray.dir.dot(&intersection.normal);

        let (outward_normal, ni_over_nt, cosine) = if rdotn > 0.0 {
            let cosine = self.refraction * rdotn / ray.dir.magnitude();
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
                        origin: intersection.point(&ray) - intersection.normal * ANTI_ACNE,
                        dir: refracted,
                    },
                    self.attenuation,
                ));
            }
        }

        Some((
            Ray {
                origin: intersection.point(&ray) + intersection.normal * ANTI_ACNE,
                dir: ray.dir.reflect(&intersection.normal),
            },
            self.attenuation,
        ))
    }
}

pub struct DiffuseLight<T: Texture> {
    pub value: T,
}

impl<T: Texture> Material for DiffuseLight<T> {
    fn scatter(&self, _: &Ray, _: &RayIntersection) -> Option<(Ray, LinSrgb)> {
        None
    }

    fn emitted(&self, uv: Option<na::Point2<Scalar>>, p: Point) -> LinSrgb {
        self.value.sample(uv, p)
    }
}

pub struct Isotropic<T: Texture> {
    pub albedo: T,
}

impl<T: Texture> Material for Isotropic<T> {
    fn scatter(&self, ray: &Ray, intersection: &RayIntersection) -> Option<(Ray, LinSrgb)> {
        Some((
            Ray {
                origin: intersection.point(&ray) + intersection.normal * ANTI_ACNE,
                dir: rand_in_sphere(),
            },
            self.albedo.sample_using_intersection(&ray, &intersection),
        ))
    }
}

pub fn refract(v: &Vector, n: &Vector, ni_over_nt: Scalar) -> Option<Vector> {
    let uv = v.normalize();
    let dt = uv.dot(&n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if discriminant > 0.0 {
        Some(ni_over_nt * (uv - n * dt) - n * discriminant.sqrt())
    } else {
        None
    }
}

pub fn schlick(cosine: Scalar, refraction: Scalar) -> Scalar {
    let r0 = (1.0 - refraction) / (1.0 + refraction);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}
