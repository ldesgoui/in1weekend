use crate::prelude::*;

pub trait Texture {
    fn sample(&self, uv: Option<na::Point2<Scalar>>, p: Point) -> LinSrgb;
    fn sample_using_intersection(&self, ray: &Ray, intersection: &RayIntersection) -> LinSrgb {
        self.sample(intersection.uvs, intersection.point(&ray))
    }
}

impl Texture for LinSrgb {
    fn sample(&self, _: Option<na::Point2<Scalar>>, _: Point) -> LinSrgb {
        *self
    }
}

pub struct Checkerboard<E: Texture, O: Texture> {
    pub even: E,
    pub odd: O,
    pub size: Scalar,
}

impl<E: Texture, O: Texture> Texture for Checkerboard<E, O> {
    fn sample(&self, uv: Option<na::Point2<Scalar>>, p: Point) -> LinSrgb {
        let p = p * self.size;
        if p.x.sin() * p.y.sin() * p.z.sin() < 0.0 {
            self.odd.sample(uv, p)
        } else {
            self.even.sample(uv, p)
        }
    }
}

pub struct UVTexture;

impl Texture for UVTexture {
    // TODO: wrap
    fn sample(&self, uv: Option<na::Point2<Scalar>>, _: Point) -> LinSrgb {
        if let Some(uv) = uv {
            LinSrgb::new(na::wrap(uv.x, 0.0, 1.0), na::wrap(uv.y, 0.0, 1.0), 0.0)
        } else {
            LinSrgb::default()
        }
    }
}

pub struct PointTexture;

impl Texture for PointTexture {
    // TODO: wrap
    fn sample(&self, _: Option<na::Point2<Scalar>>, p: Point) -> LinSrgb {
        LinSrgb::new(
            na::wrap(p.x, 0.0, 1.0),
            na::wrap(p.y, 0.0, 1.0),
            na::wrap(p.z, 0.0, 1.0),
        )
    }
}

pub struct NormalTexture;

impl Texture for NormalTexture {
    fn sample(&self, _: Option<na::Point2<Scalar>>, _: Point) -> LinSrgb {
        LinSrgb::default()
    }

    fn sample_using_intersection(&self, _: &Ray, intersection: &RayIntersection) -> LinSrgb {
        let n = (intersection.normal + Vector::new(1.0, 1.0, 1.0)) / 2.0;
        LinSrgb::new(n.x, n.y, n.z)
    }
}
