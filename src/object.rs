use crate::prelude::*;

// Object:
//
// to avoid shapes, materials and transforms from requiring a Box to any sub-element they need,
// we pass a Boxed Trait to the tree and implement composites through generics
// it limits the number of derefs required to 1 instead of N
//
pub trait Object: Send + Sync {
    fn aabb(&self) -> AABB;
    fn ray_cast(&self, ray: &Ray) -> Option<RayIntersection>;
    fn random_to_object(&self, from: &Point) -> Vector;
    fn pdf_value(&self, ray: &Ray, intersection: &RayIntersection) -> Scalar;
    fn material_scatter(
        &self,
        ray: &Ray,
        intersection: &RayIntersection,
        importance_sample: &Option<(Vector, Scalar)>,
    ) -> Option<(Ray, Color)>;
    fn material_emitted(&self, ray: &Ray, intersection: &RayIntersection) -> Color;
    fn important(&self) -> bool;
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
    fn aabb(&self) -> AABB {
        self.shape.aabb(&self.transform)
    }

    fn ray_cast(&self, ray: &Ray) -> Option<RayIntersection> {
        self.shape
            .as_ray_cast()?
            .toi_and_normal_and_uv_with_ray(&self.transform, ray, false)
    }

    fn random_to_object(&self, from: &Point) -> Vector {
        // TODO
        Vector::new(0., 0., 0.)
    }

    fn pdf_value(&self, _ray: &Ray, _intersection: &RayIntersection) -> Scalar {
        // TODO
        0.
    }

    fn material_scatter(
        &self,
        ray: &Ray,
        intersection: &RayIntersection,
        importance_sample: &Option<(Vector, Scalar)>,
    ) -> Option<(Ray, Color)> {
        self.material.scatter(ray, intersection, importance_sample)
    }

    fn material_emitted(&self, ray: &Ray, intersection: &RayIntersection) -> Color {
        self.material.emitted(ray, intersection)
    }

    fn important(&self) -> bool {
        self.material.important()
    }
}
