use crate::prelude::*;

pub struct Scene {
    // TODO: transform to texture?
    // figure out how to get u, v, p from ray
    // although that'll require boxed trait
    pub background: palette::Gradient<Color>,
    pub objects: BVT,
}

impl Scene {
    pub fn trace(&self, init_ray: &Ray) -> Color {
        let mut ray = *init_ray;
        let mut color = Color::default();
        let mut attenuation = Color::new(1., 1., 1.);

        for _ in 0..50 {
            let search_result = self
                .objects
                .best_first_search(&mut CostByRayCast { ray: &ray });

            if search_result.is_none() {
                let background = self.background.get((ray.dir.normalize().y + 1.) / 2.);
                color = color + attenuation * background;
                break;
            }

            let (object, intersection) = search_result.unwrap();

            let emitted = object.material_emitted(&ray, &intersection);
            color = color + attenuation * emitted;

            let scatter_result = object.material_scatter(&ray, &intersection);

            if scatter_result.is_none() {
                break;
            }

            let (scatter_ray, scatter_attenuation) = scatter_result.unwrap();
            ray = scatter_ray;
            attenuation = attenuation * scatter_attenuation;

            if attenuation.red + attenuation.green + attenuation.blue < 0.0003 {
                break;
            }
        }
        color
    }
}
