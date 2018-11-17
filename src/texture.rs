use crate::prelude::*;

pub trait Texture {
    fn sample(&self, ray: &Ray, intersection: &RayIntersection) -> Color {
        self.sample_from_normal(intersection.normal)
            + intersection
                .uvs
                .map(|uvs| self.sample_from_uv(uvs))
                .unwrap_or(Color::default())
            + self.sample_from_point(intersection.point(&ray))
    }

    fn sample_from_uv(&self, _uvs: na::Point2<Scalar>) -> Color {
        Color::default()
    }

    fn sample_from_normal(&self, _normal: Vector) -> Color {
        Color::default()
    }

    fn sample_from_point(&self, _point: Point) -> Color {
        Color::default()
    }
}

impl Texture for Color {
    fn sample(&self, _: &Ray, _: &RayIntersection) -> Color {
        *self
    }
}

// COMBINATORS

pub struct Checkerboard<E: Texture, O: Texture> {
    pub even: E,
    pub odd: O,
    pub size: Scalar,
}

impl<E: Texture, O: Texture> Texture for Checkerboard<E, O> {
    fn sample(&self, ray: &Ray, intersection: &RayIntersection) -> Color {
        let p = intersection.point(&ray) * self.size;
        if p.x.sin() * p.y.sin() * p.z.sin() < 0. {
            self.odd.sample(ray, intersection)
        } else {
            self.even.sample(ray, intersection)
        }
    }
}

// DEBUG HELPERS

pub struct DebugUV;

impl Texture for DebugUV {
    fn sample_from_uv(&self, uv: na::Point2<Scalar>) -> Color {
        palette::Srgb::new(na::wrap(uv.x, 0., 1.), na::wrap(uv.y, 0., 1.), 0.).into_linear()
    }
}

pub struct DebugPoint;

impl Texture for DebugPoint {
    fn sample_from_point(&self, p: Point) -> Color {
        palette::Srgb::new(
            na::wrap(p.x, 0., 1.),
            na::wrap(p.y, 0., 1.),
            na::wrap(p.z, 0., 1.),
        )
        .into_linear()
    }
}

pub struct DebugNormal;

impl Texture for DebugNormal {
    fn sample_from_normal(&self, normal: Vector) -> Color {
        let n = (normal + Vector::new(1., 1., 1.)) / 2.;
        palette::Srgb::new(n.x, n.y, n.z).into_linear()
    }
}
