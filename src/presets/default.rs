use crate::prelude::*;

pub fn camera() -> Camera {
    Camera::new(
        &Point::new(1., 1., 1.),
        &Point::new(0., 0., -1.),
        &Vector::y().into(),
        90.,
        0.,
        None,
        1. / 500.,
        na::Vector2::new(500, 500),
        50,
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
                albedo: Noise2D {
                    noise: Perlin::new(),
                    gradient: Gradient::new(vec![
                        Color::new(1., 0., 0.),
                        Color::new(0., 1., 0.),
                        Color::new(0., 0., 1.)
                    ]),
                }
            },
            translation: Vector::new(0., 0., -1.),
        }, {
            shape: Cuboid::new(Vector::new(1., 1., 1.)),
            material: Metal {
                fuzz: 0.1,
                albedo: Color::new(0.8, 0.6, 0.3),
            },
            translation: Vector::new(-3., 0., -1.),
        }],
    }
}
