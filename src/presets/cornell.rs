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
        na::Vector2::new(2000, 2000),
        1000,
    )
}

pub fn scene() -> Scene {
    mkScene! {
        background: [ Color::new(0., 0., 0.) ],
        objects: [{ // back
            shape: nc::shape::Cuboid::new(Vector::new(1., 1., 0.5)),
            material: Lambertian {
                albedo: palette::Srgb::new(0.73, 0.73, 0.73).into_linear(),
            },
            translation: Vector::new(0., 0., 1.),
        }, { // top
            shape: nc::shape::Cuboid::new(Vector::new(1., 0.5, 1.)),
            material: Lambertian {
                albedo: palette::Srgb::new(0.73, 0.73, 0.73).into_linear(),
            },
            translation: Vector::new(0., 1., 0.),
        }, { // bottom
            shape: nc::shape::Cuboid::new(Vector::new(1., 0.5, 1.)),
            material: Lambertian {
                albedo: palette::Srgb::new(0.73, 0.73, 0.73).into_linear(),
            },
            translation: Vector::new(0., -1., 0.),
        }, { // left green
            shape: nc::shape::Cuboid::new(Vector::new(0.5, 1., 1.)),
            material: Lambertian {
                albedo: palette::Srgb::new(0.12, 0.45, 0.15).into_linear(),
            },
            translation: Vector::new(1., 0., 0.),
        }, { // right red
            shape: nc::shape::Cuboid::new(Vector::new(0.5, 1., 1.)),
            material: Lambertian {
                albedo: palette::Srgb::new(0.65, 0.5, 0.5).into_linear(),
            },
            translation: Vector::new(-1., 0., 0.),
        }, { // top light
            shape: nc::shape::Cuboid::new(Vector::new(0.5, 0.5, 0.5)),
            material: DiffuseLight {
                value: Color::new(15., 15., 15.)
            },
            translation: Vector::new(0., 0.95, 0.),
        }, { // smaller object
            shape: ConstantMedium {
                shape: nc::shape::Ball::new(0.3),
                density: 3.,
            },
            material: Isotropic {
                albedo: Color::new(0., 0., 1.),
            },
            translation: Vector::new(-0.3, -0.6, -0.3),
        }, { // bigger object
            // shape: nc::shape::Ball::new(0.6),
            shape: nc::shape::Cuboid::new(Vector::new(0.3, 0.6, 0.3)),
            material: Dielectric {
                attenuation: Color::new(1., 1., 1.),
                refraction: 1.52,
            },
            translation: Vector::new(0.3, -0.4, 0.3),
        }],
    }
}
