#![allow(dead_code)]
#![feature(duration_as_u128)]
#![feature(extern_crate_item_prelude)]
#![feature(trace_macros)]

pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub extern crate image;
pub extern crate indicatif;
pub extern crate nalgebra as na;
pub extern crate ncollide3d as nc;
pub extern crate palette;
pub extern crate rand;
pub extern crate rayon;

pub mod camera;
#[macro_use]
pub mod macros;
pub mod material;
pub mod object;
pub mod presets;
pub mod scene;
pub mod shape;
pub mod texture;

pub mod prelude {
    pub use crate::camera::Camera;
    pub use crate::material::*;
    pub use crate::object::Object;
    pub use crate::scene::Scene;
    pub use crate::shape::*;
    pub use crate::texture::*;
    pub use crate::*;
    pub use palette::LinSrgb;

    pub type AABB = nc::bounding_volume::AABB<Scalar>;
    pub type BVT = nc::partitioning::BVT<Box<Object>, AABB>;
    pub type Isometry = na::Isometry3<Scalar>;
    pub type Point = na::Point3<Scalar>;
    pub type Ray = nc::query::Ray<Scalar>;
    pub type RayIntersection = nc::query::RayIntersection<Scalar>;
    pub type RayCast = nc::query::RayCast<Scalar>;
    pub type Scalar = f32;
    pub type Shape = nc::shape::Shape<Scalar>;
    pub type Vector = na::Vector3<Scalar>;
    pub type UnitVector = na::Unit<na::Vector3<Scalar>>;
}

// random utils

use crate::prelude::*;

pub struct CostByRayCast<'a> {
    ray: &'a Ray,
}

pub trait Reflect {
    fn reflect(&self, normal: &Vector) -> Self;
}

impl Reflect for Vector {
    fn reflect(&self, normal: &Vector) -> Self {
        self - 2.0 * self.dot(&normal) * normal
    }
}

pub trait RayIntersectionPoint {
    fn point(&self, ray: &Ray) -> Point;
}

impl RayIntersectionPoint for RayIntersection {
    fn point(&self, ray: &Ray) -> Point {
        ray.origin + ray.dir * self.toi
    }
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

pub fn rand_in_disk() -> Point {
    let theta: Scalar = 2.0 * std::f32::consts::PI * rand::random::<Scalar>();
    let (x, y) = theta.sin_cos();

    Point::new(x, y, 0.0)
}

pub fn rand_in_sphere() -> Vector {
    // TODO: verify speed and validity
    // rand::random::<Vector>().normalize() * rand::random::<Scalar>().cbrt()

    loop {
        let v = 2.0 * rand::random::<Vector>() - Vector::new(1.0, 1.0, 1.0);
        if v.magnitude_squared() < 1.0 {
            return v;
        }
    }
}
