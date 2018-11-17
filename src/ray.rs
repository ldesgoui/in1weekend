use crate::prelude::*;

pub trait RayIntersectionPoint {
    fn point(&self, ray: &Ray) -> Point;
}

impl RayIntersectionPoint for RayIntersection {
    fn point(&self, ray: &Ray) -> Point {
        ray.origin + ray.dir * self.toi
    }
}

pub struct CostByRayCast<'a> {
    pub ray: &'a Ray,
}

impl<'a> nc::partitioning::BVTCostFn<Scalar, Box<Object>, AABB> for CostByRayCast<'a> {
    type UserData = RayIntersection;

    fn compute_bv_cost(&mut self, bv: &AABB) -> Option<Scalar> {
        use ncollide3d::query::RayCast;

        bv.toi_with_ray(&Isometry::identity(), self.ray, true)
    }

    fn compute_b_cost(&mut self, b: &Box<Object>) -> Option<(Scalar, Self::UserData)> {
        b.ray_cast(self.ray).map(|i| (i.toi, i))
    }
}
