use crate::prelude::*;

// Object:
//
// to avoid shapes, materials and transforms from requiring a Box to any sub-element they need,
// we pass a Boxed Trait to the tree and implement composites through generics
// it limits the number of derefs required to 1 instead of N
//
pub trait Object: Send + Sync {
    fn ray_cast(&self, ray: &Ray) -> Option<RayIntersection>;
    fn material_scatter(&self, ray: &Ray, intersection: &RayIntersection)
        -> Option<(Ray, LinSrgb)>;
    fn material_emitted(&self, ray: &Ray, intersection: &RayIntersection) -> LinSrgb;
    fn aabb(&self) -> AABB;
}

pub struct ObjectInner<M, S>
where
    M: Material + Sync + Send,
    S: nc::shape::Shape<Scalar>,
{
    pub material: M,
    pub shape: S,
    pub transform: Isometry,
}

impl<M, S> Object for ObjectInner<M, S>
where
    M: Material + Sync + Send,
    S: nc::shape::Shape<Scalar>,
{
    fn ray_cast(&self, ray: &Ray) -> Option<RayIntersection> {
        self.shape
            .as_ray_cast()?
            .toi_and_normal_and_uv_with_ray(&self.transform, ray, false)
    }
    fn material_scatter(
        &self,
        ray: &Ray,
        intersection: &RayIntersection,
    ) -> Option<(Ray, LinSrgb)> {
        self.material.scatter(ray, intersection)
    }
    fn material_emitted(&self, ray: &Ray, intersection: &RayIntersection) -> LinSrgb {
        self.material.emitted_using_intersection(ray, intersection)
    }
    fn aabb(&self) -> AABB {
        self.shape.aabb(&self.transform)
    }
}
