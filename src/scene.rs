use crate::prelude::*;

pub struct Scene {
    // TODO: transform to texture?
    // figure out how to get u, v, p from ray
    // although that'll require boxed trait
    pub background: palette::Gradient<Color>,
    pub objects: BVT,
}

const MAX_ITER: u32 = 50;

impl Scene {
    pub fn trace(&self, ray: &Ray, iter: u32) -> Color {
        match self
            .objects
            .best_first_search(&mut CostByRayCast { ray: &ray })
        {
            None => self.background.get((ray.dir.normalize().y + 1.) / 2.),
            Some((object, intersection)) => {
                let emitted = object.material_emitted(&ray, &intersection);

                if iter >= MAX_ITER {
                    return emitted;
                }

                match object.material_scatter(
                    &ray,
                    &intersection,
                    &None, //&self.importance_sample(&intersection.point_nudged_out(&ray), &object),
                ) {
                    None => emitted,
                    Some((scattered, attenuation)) => {
                        emitted + attenuation * self.trace(&scattered, iter + 1)
                    }
                }
            }
        }
    }

    fn importance_sample(&self, from: &Point, ignored: &Box<Object>) -> Option<(Vector, Scalar)> {
        use rand::Rng;

        let mut important_objects = Vec::new();
        self.objects.visit(&mut ImportantObjectCollector {
            ignored: ignored,
            collector: &mut important_objects,
        });

        let chosen_one = rand::thread_rng().choose(&important_objects[..])?;

        let direction = from.coords - unsafe { (**chosen_one).random_in_object() };

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
