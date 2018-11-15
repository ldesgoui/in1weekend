use crate::prelude::*;

pub fn camera() -> Camera {
    Camera::new(
        &Point::new(1.0, 1.0, 1.0),
        &Point::new(0.0, 0.0, -1.0),
        &Vector::y().into(),
        90.0,
        0.0,
        None,
        1.0 / 500.0,
        na::Vector2::new(500, 500),
        200,
    )
}

pub fn scene() -> Scene {
    mkScene! {
        objects: [{ // floor
            shape: nc::shape::Ball::new(100.0),
            material: Lambertian {
                albedo: Checkerboard {
                    odd: LinSrgb::new(1.0, 1.0, 1.0),
                    even: LinSrgb::new(0.0, 0.0, 0.0),
                    size: 10.0,
                },
            },
            translation: Vector::new(0.0, -100.5, 0.0),
        }, {
            shape: nc::shape::Ball::new(1.0),
            //shape: nc::shape::Cuboid::new(Vector::new(1.0, 1.0, 1.0)),
            material: Dielectric {
                refraction: 1.52,
                attenuation: LinSrgb::new(1.0, 1.0, 1.0),
            },
            translation: Vector::new(0.0, 0.0, -1.0),
        }, {
            shape: nc::shape::Cuboid::new(Vector::new(1.0, 1.0, 1.0)),
            material: Metal {
                fuzz: 0.0,
                albedo: LinSrgb::new(0.8, 0.6, 0.3),
            },
            translation: Vector::new(-3.0, 0.0, -1.0),
        }],
    }
}
