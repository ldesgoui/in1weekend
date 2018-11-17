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
    mkScene! {
        objects: [{ // floor
            shape: nc::shape::Ball::new(100.),
            material: Lambertian {
                albedo: Checkerboard {
                    odd: Color::new(1., 1., 1.),
                    even: Color::new(0., 0., 0.),
                    size: 10.,
                },
            },
            translation: Vector::new(0., -100.5, 0.),
        }, {
            shape: nc::shape::Ball::new(1.),
            material: Dielectric {
                refraction: 1.52,
                attenuation: Color::new(1., 1., 1.),
            },
            translation: Vector::new(0., 0., -1.),
        }, {
            shape: nc::shape::Cuboid::new(Vector::new(1., 1., 1.)),
            material: Metal {
                fuzz: 0.1,
                albedo: Color::new(0.8, 0.6, 0.3),
            },
            translation: Vector::new(-3., 0., -1.),
        }],
    }
}
