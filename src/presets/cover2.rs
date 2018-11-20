use crate::prelude::*;

pub fn camera() -> Camera {
    Camera::new(
        &Point::new(0., 2., 0.),
        &Point::new(1., 2., -6.),
        &Vector::y().into(),
        90.,
        0.0,
        None,
        1. / 500.,
        na::Vector2::new(500, 500),
        1000,
    )
}

pub fn scene() -> Scene {
    use nc::shape::*;
    use rand::Rng;

    let mut objects: Vec<(Box<Object>, AABB)> = Vec::new();

    objects.push(mkObject!({
        shape: Cuboid::new(Vector::new(2., 2., 2.)),
        material: DiffuseLight { value: Color::new(7., 7., 7.) },
        translation: Vector::new(0., 7., -5.),
    }));

    objects.push(mkObject!({
        shape: ConstantMedium {
            shape: Ball::new(1.),
            density: 0.2,
        },
        material: Isotropic { albedo: Color::new(1., 1., 1.) },
        translation: Vector::new(2., 1., -5.),
    }));

    objects.push(mkObject!({
        shape: Ball::new(1.),
        material: Lambertian {
            albedo: Noise3D {
                noise: noise::Perlin::new(),
                scale: Vector::new(1., 5., 1.),
                gradient: palette::Gradient::new(vec![
                    Color::new(1., 0., 0.),
                    Color::new(0., 0., 1.),
                ]),
            }
        },
        translation: Vector::new(-1., 1., -4.),
    }));

    for x in -20..20 {
        for z in -40..0 {
            objects.push(mkObject!({
                shape: Cuboid::new(Vector::new(1., 1., 1.)),
                material: Lambertian { albedo: Color::new(0.5, 0.8, 0.5) },
                translation: Vector::new(x as f32, rand::random::<f32>() - 2., z as f32),
            }));
        }
    }

    for _ in 0..1000 {
        objects.push(mkObject!({
            shape: Ball::new(0.5),
            material: Lambertian { albedo: Color::new(1.0, 1.0, 1.0) },
            translation: Vector::new(3., 2., -5.) + Vector::new(
                rand::random::<f32>() * 2.,
                rand::random::<f32>() * 2.,
                rand::random::<f32>() * 2.,
            ),
        }));
    }

    Scene {
        background: palette::gradient::Gradient::new(vec![Color::new(0.01, 0.01, 0.01)]),
        objects: BVT::new_balanced(objects),
    }
}
