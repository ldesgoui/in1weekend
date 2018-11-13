extern crate image;
extern crate log;
extern crate nalgebra as na;
extern crate pretty_env_logger;

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() -> Result<(), failure::Error> {
    pretty_env_logger::init();

    let width = 1000;
    let height = 500;

    info!("version: {}", VERSION);

    info!(
        "rendering an image of dimensions {} by {} pixels",
        width, height
    );
    let buf = image::RgbImage::from_fn(width, height, |x, y| color(width, height, x, y));

    info!("writing to out.png");
    buf.save("out.png")?;

    info!("displaying result");
    std::process::Command::new("feh")
        .args(&["out.png"])
        .status()?;

    warn!("success, exiting");
    Ok(())
}

fn color(width: u32, height: u32, x: u32, y: u32) -> image::Rgb<u8> {
    let r: u8 = ((x as f32 / width as f32) * u8::max_value() as f32) as u8;
    let g: u8 = ((y as f32 / height as f32) * u8::max_value() as f32) as u8;
    let b: u8 = u8::max_value();

    image::Rgb([r, g, b])
}
