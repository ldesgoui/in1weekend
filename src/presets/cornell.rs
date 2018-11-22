use crate::prelude::*;

pub fn camera() -> Camera {
    Camera::new(
        &Point::new(0., 0., -3.75),
        &Point::new(0., 0., 1.),
        &Vector::y().into(),
        40.,
        0.,
        None,
        1. / 500.,
        na::Vector2::new(200, 200),
        200,
    )
}

pub fn scene() -> Scene {
    mkScene! {
        background: [ Color::new(0., 0., 0.) ],
        objects: [{ // back
            shape: nc::shape::Cuboid::new(Vector::new(1., 1., 0.05)),
            material: Lambertian {
                albedo: palette::Srgb::new(0.73, 0.73, 0.73).into_linear(),
            },
            translation: Vector::new(0., 0., 1.),
        }, { // top
            shape: nc::shape::Cuboid::new(Vector::new(1., 0.05, 1.)),
            material: Lambertian {
                albedo: palette::Srgb::new(0.73, 0.73, 0.73).into_linear(),
            },
            translation: Vector::new(0., 1., 0.),
        }, { // bottom
            shape: nc::shape::Cuboid::new(Vector::new(1., 0.05, 1.)),
            material: Lambertian {
                albedo: palette::Srgb::new(0.73, 0.73, 0.73).into_linear(),
            },
            translation: Vector::new(0., -1., 0.),
        }, { // left green
            shape: nc::shape::Cuboid::new(Vector::new(0.05, 1., 1.)),
            material: Lambertian {
                albedo: palette::Srgb::new(0.12, 0.45, 0.15).into_linear(),
            },
            translation: Vector::new(1., 0., 0.),
        }, { // right red
            shape: nc::shape::Cuboid::new(Vector::new(0.05, 1., 1.)),
            material: Lambertian {
                albedo: palette::Srgb::new(0.65, 0.05, 0.05).into_linear(),
            },
            translation: Vector::new(-1., 0., 0.),
        }, { // top light
            shape: nc::shape::Ball::new(0.1),
            material: DiffuseLight {
                value: Color::new(15., 15., 15.)
            },
            translation: Vector::new(0., 0.6, 0.),
        }, { // smaller object
            shape: nc::shape::Ball::new(0.3),
            material: Lambertian {
                albedo: Color::new(0.6, 0.8, 0.7),
            },
            translation: Vector::new(-0.3, -0.6, -0.3),
        }, { // bigger object
            // shape: nc::shape::Ball::new(0.6),
            shape: nc::shape::Cuboid::new(Vector::new(0.3, 0.6, 0.3)),
            material: Lambertian {
                albedo: Color::new(1., 1., 1.),
            },
            translation: Vector::new(0.3, -0.4, 0.3),
        }],
    }
}
