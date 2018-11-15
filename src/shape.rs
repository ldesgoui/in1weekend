use crate::prelude::*;

pub struct ConstantMedium<S: nc::shape::Shape<Scalar>> {
    pub shape: S,
    pub density: Scalar,
}

impl<S: nc::shape::Shape<Scalar>> nc::shape::Shape<Scalar> for ConstantMedium<S> {
    fn aabb(&self, m: &Isometry) -> AABB {
        self.shape.aabb(m)
    }

    fn as_ray_cast(&self) -> Option<&RayCast> {
        Some(self)
    }
}

impl<S: nc::shape::Shape<Scalar>> nc::query::RayCast<Scalar> for ConstantMedium<S> {
    fn toi_and_normal_with_ray(
        &self,
        m: &Isometry,
        ray: &Ray,
        solid: bool,
    ) -> Option<RayIntersection> {
        let intersection1 = self
            .shape
            .as_ray_cast()?
            .toi_and_normal_with_ray(m, ray, solid)?;
        let new_ray = Ray {
            origin: intersection1.point(&ray) + (ray.dir * 0.0001),
            dir: ray.dir,
        };
        let intersection2 = self
            .shape
            .as_ray_cast()?
            .toi_and_normal_with_ray(m, &new_ray, solid)?;
        let distance_through = intersection2.toi * ray.dir.magnitude();
        let hit_distance = -(1.0 / self.density) * rand::random::<Scalar>().ln();
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
