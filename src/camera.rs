use crate::prelude::*;

#[derive(Debug)]
pub struct Camera {
    origin: Point,
    top_left_corner: Point,
    horizontal: Vector,
    vertical: Vector,
    u: Vector,
    v: Vector,

    lens_radius: Scalar,
    shutter_speed: Scalar,

    resolution: na::Vector2<u32>,
    samples: u32,
}

impl Default for Camera {
    fn default() -> Self {
        presets::default::camera()
    }
}

impl Camera {
    pub fn new(
        from: &Point,
        at: &Point,
        up: &Vector,
        vfov: Scalar,
        aperture: Scalar,
        focus_dist: Option<Scalar>,
        shutter_speed: Scalar,
        resolution: na::Vector2<u32>,
        samples: u32,
    ) -> Self {
        let theta = vfov.to_radians();
        let half_height = (theta / 2.0).tan();
        let half_width = (resolution.x as f32 / resolution.y as f32) * half_height;
        let w = (from - at).normalize();
        let u = up.cross(&w).normalize();
        let v = w.cross(&u);
        let focus_dist = focus_dist.unwrap_or((at - from).magnitude());
        Self {
            origin: *from,
            top_left_corner: from - half_width * focus_dist * u + half_height * focus_dist * v
                - focus_dist * w,
            horizontal: 2.0 * half_width * focus_dist * u,
            vertical: 2.0 * half_height * focus_dist * v,
            u: u,
            v: v,
            lens_radius: aperture / 2.0,
            shutter_speed: shutter_speed,
            resolution: resolution,
            samples: samples,
        }
    }

    pub fn bench(&self, scene: &Scene, pixels: u32) {
        use rand::Rng;

        debug!("benchmarking {:?} random pixels", pixels);

        let before_render = std::time::Instant::now();
        let mut rng = rand::thread_rng();
        let mut img = image::RgbImage::new(1, 1);

        for _ in 0..pixels {
            let color = self.capture_pixel(
                scene,
                rng.gen_range(0, self.resolution.x),
                rng.gen_range(0, self.resolution.y),
            );
            img.put_pixel(0, 0, color);
        }

        debug!(
            "results: {:?} ({:?} samples/second)",
            before_render.elapsed(),
            (pixels * self.samples) as f32
                / (before_render.elapsed().as_nanos() as f32 / 1_000_000_000.0)
        );
        info!(
            "estimating for full render: {:?}",
            (before_render.elapsed() / pixels) * self.resolution.x * self.resolution.y
        );
    }

    pub fn capture(&self, scene: &Scene) -> image::RgbImage {
        // TODO: parallelism
        // TODO: generate chunks and stitch chunks
        // (I assume this helps data locality? generating a
        // big image with low sample amount is much slower than
        // generating a small image with large sample amount)
        let before_render = std::time::Instant::now();
        let mut last_print = std::time::Instant::now();
        let second = std::time::Duration::from_secs(1);
        let total_pixels = self.resolution.x * self.resolution.y;

        let ret = image::RgbImage::from_fn(self.resolution.x, self.resolution.y, |x, y| {
            if last_print.elapsed() > second {
                let pixels = y * self.resolution.x + x;
                info!(
                    "progress: {:5.1}% in {:-3.1?}",
                    100.0 * pixels as f32 / total_pixels as f32,
                    elapsed = before_render.elapsed(),
                );
                last_print = std::time::Instant::now();
            }
            self.capture_pixel(scene, x, y)
        });
        info!(
            "total render: {:?} ({:?} p/s)",
            before_render.elapsed(),
            total_pixels as f32 / (before_render.elapsed().as_nanos() as f32 / 1_000_000_000.0)
        );
        ret
    }

    fn capture_pixel(&self, scene: &Scene, x: u32, y: u32) -> image::Rgb<u8> {
        use rayon::iter::IntoParallelIterator;
        use rayon::iter::ParallelIterator;
        // TODO: shutter speed / motion blur
        // TODO: extract sample distribution ?

        let color = (0..self.samples)
            .into_iter()
            .map(|_| {
                let u = (rand::random::<f32>() + x as f32) / self.resolution.x as f32;
                let v = (rand::random::<f32>() + y as f32) / self.resolution.y as f32;
                scene.trace(&self.ray(u, v), 0)
            })
            .fold(LinSrgb::new(0.0, 0.0, 0.0), |a, b| a + b)
            / self.samples as f32;

        let srgb: palette::Srgb<u8> = palette::Srgb::from_linear(color).into_format();

        use palette::Pixel;
        image::Rgb {
            data: *srgb.as_raw(),
        }
    }

    fn ray(&self, u: f32, v: f32) -> Ray {
        let rd = self.lens_radius * rand_in_disk();
        let offset = self.u * rd.x + self.v * rd.y;

        Ray {
            origin: self.origin + offset,
            dir: self.top_left_corner + u * self.horizontal
                - v * self.vertical
                - self.origin
                - offset,
        }
    }
}
