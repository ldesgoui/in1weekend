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

        loop {
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

            let scatter_result = object.material_scatter(
                &ray,
                &intersection,
                &self.importance_sample(&intersection.point_nudged_out(&ray), &object),
            );

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

    fn importance_sample(&self, from: &Point, ignored: &Box<Object>) -> Option<(Vector, Scalar)> {
        use rand::Rng;

        let mut important_objects = Vec::new();
        self.objects.visit(&mut ImportantObjectCollector {
            ignored: ignored,
            collector: &mut important_objects,
        });

        let chosen_one = rand::thread_rng().choose(&important_objects[..])?;

        let direction = unsafe { (**chosen_one).random_to_object(from) };

        let mut pdf_sum = 0.;
        self.objects.visit(&mut ImportantObjectPDFSum {
            ignored: ignored,
            ray: &Ray::new(*from, direction),
            accumulator: &mut pdf_sum,
        });

        pdf_sum /= important_objects.len() as Scalar;

        Some((direction, pdf_sum))
    }
}

struct ImportantObjectCollector<'a> {
    ignored: &'a Box<Object>,
    collector: &'a mut Vec<*const Box<Object>>,
}

impl<'a> nc::partitioning::BVTVisitor<Box<Object>, AABB> for ImportantObjectCollector<'a> {
    fn visit_internal(&mut self, _: &AABB) -> bool {
        true
    }

    fn visit_leaf(&mut self, object: &Box<Object>, _: &AABB) {
        let ignored_ptr: *const Object = self.ignored.as_ref();
        let object_ptr: *const Object = object.as_ref();

        if object_ptr != ignored_ptr && object.important() {
            self.collector.push(object)
        }
    }
}

struct ImportantObjectPDFSum<'a> {
    ignored: &'a Box<Object>,
    ray: &'a Ray,
    accumulator: &'a mut Scalar,
}

impl<'a> nc::partitioning::BVTVisitor<Box<Object>, AABB> for ImportantObjectPDFSum<'a> {
    fn visit_internal(&mut self, aabb: &AABB) -> bool {
        use nc::query::RayCast;

        aabb.intersects_ray(&Isometry::identity(), self.ray)
    }

    fn visit_leaf(&mut self, object: &Box<Object>, _: &AABB) {
        let ignored_ptr: *const Object = self.ignored.as_ref();
        let object_ptr: *const Object = object.as_ref();

        if object_ptr == ignored_ptr || !object.important() {
            return;
        }

        if let Some(intersection) = object.ray_cast(self.ray) {
            *self.accumulator += object.pdf_value(&self.ray, &intersection);
        }
    }
}
