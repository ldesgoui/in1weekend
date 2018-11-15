extern crate in1weekend;

use in1weekend::prelude::*;

fn main() -> Result<(), failure::Error> {
    pretty_env_logger::init();
    info!("version: {}", in1weekend::VERSION);

    warn!("creating scene");
    let scene = Scene::default();

    warn!("grabbing camera");
    let camera = Camera::default();

    warn!("checking it");
    camera.bench(&scene, 1000);

    warn!("capturing");
    let picture = camera.capture(&scene);

    warn!("saving");
    picture.save("out.png")?;

    warn!("viewing result");
    std::process::Command::new("feh")
        .args(&["-F", "out.png"])
        .status()?;

    warn!("success, exiting");
    Ok(())
}
