use crate::prelude::*;

pub fn camera() -> Camera {
    Camera::new(
        &Point::new(0.0, 0.0, -3.75),
        &Point::new(0.0, 0.0, 1.0),
        &Vector::y().into(),
        40.0,
        0.0,
        None,
        1.0 / 500.0,
        na::Vector2::new(1000, 1000),
        500,
    )
}

pub fn scene() -> Scene {
    mkScene! {
        background: [ LinSrgb::new(0.0, 0.0, 0.0) ],
        objects: [{ // back
            shape: nc::shape::Cuboid::new(Vector::new(1.0, 1.0, 0.05)),
            material: Lambertian {
                albedo: palette::Srgb::new(0.73, 0.73, 0.73).into_linear(),
            },
            translation: Vector::new(0.0, 0.0, 1.0),
        }, { // top
            shape: nc::shape::Cuboid::new(Vector::new(1.0, 0.05, 1.0)),
            material: Lambertian {
                albedo: palette::Srgb::new(0.73, 0.73, 0.73).into_linear(),
            },
            translation: Vector::new(0.0, 1.0, 0.0),
        }, { // bottom
            shape: nc::shape::Cuboid::new(Vector::new(1.0, 0.05, 1.0)),
            material: Lambertian {
                albedo: palette::Srgb::new(0.73, 0.73, 0.73).into_linear(),
            },
            translation: Vector::new(0.0, -1.0, 0.0),
        }, { // left green
            shape: nc::shape::Cuboid::new(Vector::new(0.05, 1.0, 1.0)),
            material: Lambertian {
                albedo: palette::Srgb::new(0.12, 0.45, 0.15).into_linear(),
            },
            translation: Vector::new(1.0, 0.0, 0.0),
        }, { // right red
            shape: nc::shape::Cuboid::new(Vector::new(0.05, 1.0, 1.0)),
            material: Lambertian {
                albedo: palette::Srgb::new(0.65, 0.05, 0.05).into_linear(),
            },
            translation: Vector::new(-1.0, 0.0, 0.0),
        }, { // top light
            shape: nc::shape::Cuboid::new(Vector::new(0.5, 0.05, 0.5)),
            material: DiffuseLight {
                value: LinSrgb::new(15.0, 15.0, 15.0)
            },
            translation: Vector::new(0.0, 0.95, 0.0),
        }, { // smaller object
            shape: ConstantMedium {
                shape: nc::shape::Ball::new(0.3),
                density: 3.0,
            },
            material: Isotropic {
                albedo: LinSrgb::new(0.0, 0.0, 1.0),
            },
            translation: Vector::new(-0.3, -0.6, -0.3),
        }, { // bigger object
            // shape: nc::shape::Ball::new(0.6),
            shape: nc::shape::Cuboid::new(Vector::new(0.3, 0.6, 0.3)),
            material: Dielectric {
                attenuation: LinSrgb::new(1.0, 1.0, 1.0),
                refraction: 1.52,
            },
            translation: Vector::new(0.3, -0.4, 0.3),
        }],
    }
}
