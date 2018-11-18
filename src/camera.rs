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
        let half_height = (theta / 2.).tan();
        let half_width = (resolution.x as f32 / resolution.y as f32) * half_height;
        let w = (from - at).normalize();
        let u = up.cross(&w).normalize();
        let v = w.cross(&u);
        let focus_dist = focus_dist.unwrap_or((at - from).magnitude());
        Self {
            origin: *from,
            top_left_corner: from - half_width * focus_dist * u + half_height * focus_dist * v
                - focus_dist * w,
            horizontal: 2. * half_width * focus_dist * u,
            vertical: 2. * half_height * focus_dist * v,
            u: u,
            v: v,
            lens_radius: aperture / 2.,
            shutter_speed: shutter_speed,
            resolution: resolution,
            samples: samples,
        }
    }
    pub fn capture(&self, scene: &Scene) -> image::RgbImage {
        let started = std::time::Instant::now();
        let bar = self.create_progress_bar();

        let img = image::RgbImage::from_fn(self.resolution.x, self.resolution.y, |x, y| {
            if x == 0 {
                bar.inc(1);
                bar.set_message(&self.samples_per_second(x, y, started.elapsed()));
            }
            self.capture_pixel(scene, x, y)
        });

        bar.finish_with_message(&self.samples_per_second(
            self.resolution.x,
            self.resolution.y,
            started.elapsed(),
        ));

        img
    }

    fn capture_pixel(&self, scene: &Scene, x: u32, y: u32) -> image::Rgb<u8> {
        use palette::Pixel;
        use rayon::iter::{IntoParallelIterator, ParallelIterator};

        let color = (0..self.samples)
            .into_par_iter()
            .map(|_| {
                let u = (rand::random::<Scalar>() + x as Scalar) / self.resolution.x as Scalar;
                let v = (rand::random::<Scalar>() + y as Scalar) / self.resolution.y as Scalar;
                scene.trace(&self.ray(u, v))
            })
            .reduce(|| Color::new(0., 0., 0.), |a, b| a + b)
            / self.samples as f32;

        let srgb: palette::Srgb<u8> = palette::Srgb::from_linear(color).into_format();

        image::Rgb {
            data: *srgb.as_raw(),
        }
    }

    fn ray(&self, u: Scalar, v: Scalar) -> Ray {
        let rd = self.lens_radius * Vector2::random_on_sphere();
        let offset = self.u * rd.x + self.v * rd.y;

        Ray {
            origin: self.origin + offset,
            dir: self.top_left_corner + u * self.horizontal
                - v * self.vertical
                - self.origin
                - offset,
        }
    }

    fn create_progress_bar(&self) -> indicatif::ProgressBar {
        let bar = indicatif::ProgressBar::new(self.resolution.y as u64);
        bar.set_style(
            indicatif::ProgressStyle::default_bar()
                .template(concat!(
                    "{spinner} ",
                    "{wide_bar} ",
                    "{percent:>3}%, ",
                    "Elapsed: {elapsed_precise}, ",
                    "ETA: {eta_precise}, ",
                    "Samples per second: {msg} "
                ))
                .progress_chars("█▇▆▅▄▃▂▁ "),
        );
        bar
    }

    fn samples_per_second(&self, x: u32, y: u32, elapsed: std::time::Duration) -> String {
        let seconds: f32 = elapsed.as_secs() as f32 + (elapsed.subsec_nanos() as f32) / 1e9;
        let samples = ((x + y * self.resolution.y) * self.samples) as f32;
        let rate = samples / seconds;

        match rate {
            rate if rate >= 1e+24 => format!("{:.2} yotta", rate / 1e+24),
            rate if rate >= 1e+21 => format!("{:.2} zetta", rate / 1e+21),
            rate if rate >= 1e+18 => format!("{:.2} exa  ", rate / 1e+18),
            rate if rate >= 1e+15 => format!("{:.2} peta ", rate / 1e+15),
            rate if rate >= 1e+12 => format!("{:.2} tera ", rate / 1e+12),
            rate if rate >= 1e+09 => format!("{:.2} giga ", rate / 1e+09),
            rate if rate >= 1e+06 => format!("{:.2} mega ", rate / 1e+06),
            rate if rate >= 1e+03 => format!("{:.2} kilo ", rate / 1e+03),
            rate if rate <= 1e-20 => format!("{:.2} yocto", rate / 1e-24),
            rate if rate <= 1e-18 => format!("{:.2} zepto", rate / 1e-21),
            rate if rate <= 1e-15 => format!("{:.2} atto ", rate / 1e-18),
            rate if rate <= 1e-12 => format!("{:.2} femto", rate / 1e-15),
            rate if rate <= 1e-09 => format!("{:.2} pico ", rate / 1e-12),
            rate if rate <= 1e-06 => format!("{:.2} nano ", rate / 1e-09),
            rate if rate <= 1e-03 => format!("{:.2} micro", rate / 1e-06),
            rate if rate <= 1e-00 => format!("{:.2} milli", rate / 1e-03),
            _ => format!("{:.2}      ", rate),
        }
    }
}
