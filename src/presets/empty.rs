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
        na::Vector2::new(1000, 5000),
        100,
    )
}

pub fn scene() -> Scene {
    mkScene! {
        objects: [ ],
    }
}
