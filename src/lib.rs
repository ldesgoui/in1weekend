#![allow(dead_code)]
#![feature(duration_as_u128)]
#![feature(extern_crate_item_prelude)]

pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub extern crate image;
pub extern crate indicatif;
pub extern crate nalgebra as na;
pub extern crate ncollide3d as nc;
pub extern crate palette;
pub extern crate rand;
pub extern crate rayon;

pub mod camera;
pub mod ray;
#[macro_use]
pub mod macros;
pub mod material;
pub mod object;
pub mod presets;
pub mod scene;
pub mod shape;
pub mod texture;

pub mod prelude {
    pub use crate::camera::*;
    pub use crate::material::*;
    pub use crate::object::*;
    pub use crate::ray::*;
    pub use crate::scene::*;
    pub use crate::shape::*;
    pub use crate::texture::*;
    pub use crate::*;

    pub use palette::LinSrgb;
    pub type Color = palette::LinSrgb;
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

pub trait Reflect {
    fn reflect(&self, normal: &Vector) -> Self;
}

impl Reflect for Vector {
    fn reflect(&self, normal: &Vector) -> Self {
        self - 2.0 * self.dot(&normal) * normal
    }
}
