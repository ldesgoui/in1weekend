use crate::prelude::*;

// TODO: change with Ray energy
const MAX_DEPTH: u32 = 10;

pub struct Scene {
    // TODO: transform to texture?
    // figure out how to get u, v, p from ray
    // although that'll require boxed trait
    pub background: palette::Gradient<Color>,
    pub objects: BVT,
}

impl Scene {
    pub fn trace(&self, ray: &Ray, depth: u32) -> Color {
        // Apparently, tail-recursion is not so good in Rust
        // can we change this to a loop ?
        match self
            .objects
            .best_first_search(&mut (CostByRayCast { ray: &ray }))
        {
            None => self.background.get((ray.dir.normalize().y + 1.) / 2.),
            Some((object, intersection)) => {
                let emitted = object.material_emitted(&ray, &intersection);

                if depth > MAX_DEPTH {
                    return emitted;
                }

                if let Some((scattered, attenuation)) = object.material_scatter(
                    &ray,
                    &intersection,
                    &self.importance_sample(&intersection.point_nudged_out(&ray), &object),
                ) {
                    emitted + attenuation * self.trace(&scattered, depth + 1)
                } else {
                    emitted
                }
            }
        }
    }

    fn importance_sample(&self, from: &Point, ignored: &Box<Object>) -> Option<(Vector, Scalar)> {
        use rand::Rng;

        let important_objects: Vec<Box<Object>> = vec![]; // some kinda search ?
        let chosen_one = rand::thread_rng().choose(&important_objects[..])?;
        let direction = chosen_one.random_to_object(from);
        // can use RayInterferencesCollector here v
        // prev: important_objects.iter().map(|o| o.pdf_value(from, &direction)).sum::<Scalar>() / important_objects.len() as Scalar;
        let value = 0.;
        Some((direction, value))
    }
}
