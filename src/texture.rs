use crate::prelude::*;

pub trait Texture {
    fn sample(&self, ray: &Ray, intersection: &RayIntersection) -> Color;
}

impl Texture for Color {
    fn sample(&self, _: &Ray, _: &RayIntersection) -> Color {
        *self
    }
}

impl Texture for palette::Gradient<Color> {
    fn sample(&self, _: &Ray, intersection: &RayIntersection) -> Color {
        if let Some(uvs) = intersection.uvs {
            self.get(uvs.y)
        } else {
            Color::default()
        }
    }
}

pub struct Noise2D<N: noise::NoiseFn<[f64; 2]>> {
    pub gradient: palette::Gradient<Color>,
    pub noise: N,
    pub scale: Vector2,
}

impl<N: noise::NoiseFn<[f64; 2]>> Texture for Noise2D<N> {
    fn sample(&self, _: &Ray, intersection: &RayIntersection) -> Color {
        if let Some(uvs) = intersection.uvs {
            let value = self
                .noise
                .get([(uvs.x * self.scale.x) as f64, (uvs.y * self.scale.y) as f64]);
            self.gradient.get((value as f32 + 1.) / 2.)
        } else {
            Color::default()
        }
    }
}

pub struct Noise3D<N: noise::NoiseFn<[f64; 3]>> {
    pub gradient: palette::Gradient<Color>,
    pub noise: N,
    pub scale: Vector,
}

impl<N: noise::NoiseFn<[f64; 3]>> Texture for Noise3D<N> {
    fn sample(&self, ray: &Ray, intersection: &RayIntersection) -> Color {
        let p = intersection.point(&ray);
        let value = self.noise.get([
            (p.x * self.scale.x) as f64,
            (p.y * self.scale.y) as f64,
            (p.z * self.scale.z) as f64,
        ]);
        self.gradient.get((value as f32 + 1.) / 2.)
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
    fn sample(&self, _ray: &Ray, intersection: &RayIntersection) -> Color {
        if let Some(uv) = intersection.uvs {
            palette::Srgb::new(na::wrap(uv.x, 0., 1.), na::wrap(uv.y, 0., 1.), 0.).into_linear()
        } else {
            Color::default()
        }
    }
}

pub struct DebugPoint;

impl Texture for DebugPoint {
    fn sample(&self, ray: &Ray, intersection: &RayIntersection) -> Color {
        let p = intersection.point(&ray);
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
    fn sample(&self, _ray: &Ray, intersection: &RayIntersection) -> Color {
        let n = (intersection.normal + Vector::new(1., 1., 1.)) / 2.;
        palette::Srgb::new(n.x, n.y, n.z).into_linear()
    }
}

pub struct DebugDistance;

impl Texture for DebugDistance {
    fn sample(&self, ray: &Ray, intersection: &RayIntersection) -> Color {
        let d = intersection.toi / ray.dir.magnitude();
        palette::Srgb::new(
            na::wrap(d, 0., 1.),
            na::wrap(d, 0., 1.),
            na::wrap(d, 0., 1.),
        )
        .into_linear()
    }
}
