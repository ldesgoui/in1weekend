use crate::prelude::*;

pub fn camera() -> Camera {
    Camera::new(
        &Point::new(0., 1.5, 0.),
        &Point::new(0., 1., -6.),
        &Vector::y().into(),
        45.,
        0.1,
        None,
        1. / 500.,
        na::Vector2::new(1000, 500),
        1000,
    )
}

pub fn scene() -> Scene {
    use nc::shape::*;
    use rand::Rng;

    let mut objects: Vec<(Box<Object>, AABB)> = Vec::new();

    objects.push(mkObject!({
        shape: Ball::new(10000.),
        material: Lambertian { albedo: Color::new(0.5, 0.5, 0.5) },
        translation: Vector::new(0., -10000., 0.),
    }));

    objects.push(mkObject!({
        shape: Ball::new(1.),
        material: Lambertian { albedo: Color::new(0.4, 0.2, 0.1) },
        translation: Vector::new(-1., 1., -12.),
    }));

    objects.push(mkObject!({
        shape: Ball::new(1.),
        material: Dielectric {
            attenuation: Color::new(1., 1., 1.),
            refraction: -1.3,
        },
        translation: Vector::new(0., 1., -8.),
    }));

    objects.push(mkObject!({
        shape: Ball::new(1.),
        material: Dielectric {
            attenuation: Color::new(1., 1., 1.),
            refraction: 1.52,
        },
        translation: Vector::new(0., 1., -8.),
    }));

    objects.push(mkObject!({
        shape: Ball::new(1.),
        material: Metal {
            fuzz: 0.,
            albedo: Color::new(0.8, 0.6, 0.4),
        },
        translation: Vector::new(1., 1., -4.),
    }));

    for x in -10..10 {
        for z in -20..0 {
            let translation = Vector::new(
                x as f32 + rand::random::<f32>(),
                0.2,
                z as f32 + rand::random::<f32>(),
            );

            if rand::random() {
                objects.push(mkObject!({
                    shape: Ball::new(0.2),
                    material: Lambertian {
                        albedo: Color::new(
                            rand::thread_rng().gen_range(0., 1.),
                            rand::thread_rng().gen_range(0., 1.),
                            rand::thread_rng().gen_range(0., 1.),
                        ),
                    },
                    translation: translation,
                }));
            } else if rand::random() {
                objects.push(mkObject!({
                    shape: Ball::new(0.2),
                    material: Metal {
                        fuzz: rand::random(),
                        albedo: Color::new(
                            rand::thread_rng().gen_range(0.5, 1.),
                            rand::thread_rng().gen_range(0.5, 1.),
                            rand::thread_rng().gen_range(0.5, 1.),
                        ),
                    },
                    translation: translation,
                }));
            } else {
                objects.push(mkObject!({
                    shape: Ball::new(0.2),
                    material: Dielectric {
                        refraction: rand::thread_rng().gen_range(1.5, 3.),
                        attenuation: Color::new(
                            rand::thread_rng().gen_range(0.9, 1.),
                            rand::thread_rng().gen_range(0.9, 1.),
                            rand::thread_rng().gen_range(0.9, 1.),
                        ),
                    },
                    translation: translation,
                }));
            }
        }
    }

    Scene {
        background: palette::gradient::Gradient::new(vec![
            Color::new(0.4, 0.5, 1.),
            Color::new(1., 1., 1.),
            Color::new(0.4, 0.5, 1.),
        ]),
        objects: BVT::new_balanced(objects),
    }
}
