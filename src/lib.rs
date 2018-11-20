#![allow(dead_code)]
#![feature(duration_as_u128)]
#![feature(extern_crate_item_prelude)]

#[macro_use]
pub mod macros;

pub extern crate image;
pub extern crate indicatif;
pub extern crate nalgebra as na;
pub extern crate ncollide3d as nc;
pub extern crate noise;
pub extern crate palette;
pub extern crate rand;
pub extern crate rayon;

pub mod camera;
pub mod material;
pub mod object;
pub mod presets;
pub mod ray;
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
    pub use crate::{Reflect, SphereRandom};

    // TODO: Generics
    //          scalar for space
    //          color

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
    pub type Vector2 = na::Vector2<Scalar>;
    pub type UnitVector = na::Unit<na::Vector3<Scalar>>;
}

// random utils

use crate::prelude::*;

pub trait Reflect {
    fn reflect(&self, normal: &Vector) -> Self;
}

impl Reflect for Vector {
    fn reflect(&self, normal: &Vector) -> Self {
        self - 2. * self.dot(&normal) * normal
    }
}

pub trait SphereRandom {
    fn random_in_sphere() -> Self;
    fn random_on_sphere() -> Self;
}

impl SphereRandom for Vector {
    fn random_in_sphere() -> Self {
        Vector::random_on_sphere() * rand::random::<Scalar>()
    }

    fn random_on_sphere() -> Self {
        (rand::random::<Vector>() * 2. - Vector::new(1., 1., 1.)).normalize()
    }
}

impl SphereRandom for Vector2 {
    fn random_in_sphere() -> Self {
        Vector2::random_on_sphere() * rand::random::<Scalar>()
    }

    fn random_on_sphere() -> Self {
        (rand::random::<Vector2>() * 2. - Vector2::new(1., 1.)).normalize()
    }
}
