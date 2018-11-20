use crate::prelude::*;

pub trait Shape:
    nc::bounding_volume::HasBoundingVolume<Scalar, AABB> + nc::query::RayCast<Scalar>
{
    fn random_in_object(&self, _m: &Isometry) -> Vector;
    fn pdf_value(&self, _m: &Isometry, _ray: &Ray, _intersection: &RayIntersection) -> Scalar;
}

impl Shape for nc::shape::Ball<Scalar> {
    fn random_in_object(&self, m: &Isometry) -> Vector {
        m * (Vector::random_on_sphere() * self.radius())
    }

    fn pdf_value(&self, m: &Isometry, ray: &Ray, _intersection: &RayIntersection) -> Scalar {
        let cos_theta_max = (1.
            - self.radius() * self.radius()
                / (m.translation.vector - ray.origin.coords).magnitude_squared())
        .sqrt();
        let solid_angle = 2. * std::f32::consts::PI * (1. - cos_theta_max);
        1. / solid_angle
    }
}

impl Shape for nc::shape::Cuboid<Scalar> {
    fn random_in_object(&self, m: &Isometry) -> Vector {
        unreachable!()
        // m * Vector::random_in_sphere()
    }

    fn pdf_value(&self, _m: &Isometry, _ray: &Ray, _intersection: &RayIntersection) -> Scalar {
        unreachable!()
        //A1.
    }
}

pub struct ConstantMedium<S: Shape> {
    pub shape: S,
    pub density: Scalar,
}

impl<S: Shape> Shape for ConstantMedium<S> {
    fn random_in_object(&self, m: &Isometry) -> Vector {
        self.shape.random_in_object(m)
    }

    fn pdf_value(&self, m: &Isometry, ray: &Ray, intersection: &RayIntersection) -> Scalar {
        self.shape.pdf_value(m, ray, intersection)
    }
}

impl<S: Shape> nc::bounding_volume::HasBoundingVolume<Scalar, AABB> for ConstantMedium<S> {
    fn bounding_volume(&self, m: &Isometry) -> AABB {
        self.shape.bounding_volume(m)
    }
}

impl<S: Shape> nc::query::RayCast<Scalar> for ConstantMedium<S> {
    fn toi_and_normal_with_ray(
        &self,
        m: &Isometry,
        ray: &Ray,
        solid: bool,
    ) -> Option<RayIntersection> {
        let intersection1 = self.shape.toi_and_normal_with_ray(m, ray, solid)?;
        let new_ray = Ray {
            origin: intersection1.point_nudged_in(&ray),
            dir: ray.dir,
        };
        let intersection2 = self.shape.toi_and_normal_with_ray(m, &new_ray, solid)?;

        let distance_through = intersection2.toi * ray.dir.magnitude();
        let hit_distance = -(1. / self.density) * rand::random::<Scalar>().ln();

        if hit_distance >= distance_through {
            return None;
        }

        Some(RayIntersection {
            toi: intersection1.toi + hit_distance / ray.dir.magnitude(),
            normal: Vector::y(),
            uvs: None,
        })
    }
}
