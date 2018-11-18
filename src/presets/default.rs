use crate::prelude::*;

pub fn camera() -> Camera {
    Camera::new(
        &Point::new(1., 1., 1.),
        &Point::new(-0.5, 0., -1.),
        &Vector::y().into(),
        90.,
        0.,
        None,
        1. / 500.,
        na::Vector2::new(500, 500),
        100,
    )
}

pub fn scene() -> Scene {
    use nc::shape::*;
    use noise::Perlin;
    use palette::Gradient;

    mkScene! {
        objects: [{ // floor
            shape: Ball::new(100.),
            material: Lambertian {
                albedo: Checkerboard {
                    odd: Color::new(1., 1., 1.),
                    even: Color::new(0., 0., 0.),
                    size: 10.,
                },
            },
            translation: Vector::new(0., -100.5, 0.),
        }, {
            shape: Ball::new(1.),
            material: Lambertian {
                albedo: Gradient::new(vec![
                    Color::new(1., 0., 0.),
                    Color::new(0., 1., 0.),
                    Color::new(0., 0., 1.)
                ]),
            },
            translation: Vector::new(0., 0., -1.),
        }, {
            shape: Ball::new(1.1),
            material: Dielectric {
                attenuation: Color::new(1., 1., 1.),
                refraction: 1.52,
            },
            translation: Vector::new(0., 0., -1.),
        }, {
            shape: Cuboid::new(Vector::new(1., 1., 1.)),
            material: Metal {
                fuzz: 0.01,
                albedo: Noise3D {
                    scale: Vector::new(1., 1., 1.),
                    noise: Perlin::new(),
                    gradient: Gradient::new(vec![
                        Color::new(0., 0., 0.),
                        Color::new(0.8, 0.6, 0.3),
                        Color::new(0., 0., 0.),
                    ]),
                }
            },
            translation: Vector::new(-3., 0., -1.),
        }],
    }
}
