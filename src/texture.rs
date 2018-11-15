use crate::prelude::*;

pub trait Texture {
    fn sample(&self, u: Scalar, v: Scalar, p: Point) -> LinSrgb;
}

impl Texture for LinSrgb {
    fn sample(&self, _: Scalar, _: Scalar, _: Point) -> LinSrgb {
        *self
    }
}

pub struct Checkerboard<E: Texture, O: Texture> {
    pub even: E,
    pub odd: O,
    pub size: Scalar,
}

impl<E: Texture, O: Texture> Texture for Checkerboard<E, O> {
    fn sample(&self, u: Scalar, v: Scalar, p: Point) -> LinSrgb {
        let p = p * self.size;
        if p.x.sin() * p.y.sin() * p.z.sin() < 0.0 {
            self.odd.sample(u, v, p)
        } else {
            self.even.sample(u, v, p)
        }
    }
}

pub struct UVTexture;

impl Texture for UVTexture {
    fn sample(&self, u: Scalar, v: Scalar, _p: Point) -> LinSrgb {
        LinSrgb::new(u, v, 0.0)
    }
}

pub struct PointTexture;

impl Texture for PointTexture {
    fn sample(&self, _u: Scalar, _v: Scalar, p: Point) -> LinSrgb {
        LinSrgb::new(p.x, p.y, p.z)
    }
}
