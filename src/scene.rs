use crate::prelude::*;

// TODO: change with Ray energy
const MAX_DEPTH: u32 = 10;

pub struct Scene {
    // TODO: transform to texture?
    // figure out how to get u, v, p from ray
    // although that'll require boxed trait
    pub background: palette::gradient::Gradient<LinSrgb>,
    pub objects: std::sync::Arc<BVT>,
}

impl Scene {
    pub fn trace(&self, ray: &Ray, depth: u32) -> LinSrgb {
        // Apparently, tail-recursion is not so good in Rust
        // can we change this to a loop ?
        match self
            .objects
            .best_first_search(&mut (CostByRayCast { ray: &ray }))
        {
            None => self.background.get((ray.dir.normalize().y + 1.0) / 2.0),
            Some((object, intersection)) => {
                let emitted = object.material_emitted(&ray, &intersection);

                if depth > MAX_DEPTH {
                    return emitted;
                }
                if let Some((scattered, attenuation)) = object.material_scatter(&ray, &intersection)
                {
                    emitted + attenuation * self.trace(&scattered, depth + 1)
                } else {
                    emitted
                }
            }
        }
    }
}

impl Default for Scene {
    fn default() -> Self {
        presets::default::scene()
    }
}