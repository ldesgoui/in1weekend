extern crate clap;
extern crate in1weekend;

use in1weekend::presets::*;

fn main() -> Result<(), failure::Error> {
    let matches = clap::App::new("in1weekend")
        .arg(
            clap::Arg::with_name("PRESET")
                .multiple(true)
                .default_value("cornell"),
        )
        .get_matches();

    for preset in matches.values_of("PRESET").expect("no preset") {
        match preset {
            "cornell" => preset!(cornell),
            "cover1" => preset!(cover1),
            "cover2" => preset!(cover2),
            // "presentation" => preset!(presentation),
            _ => {
                println!("preset not found: {:?}", preset);
            }
        }
    }

    Ok(())
}

#[macro_export]
macro_rules! preset {
    ( $preset:ident ) => {{
        $preset::camera()
            .capture(&$preset::scene())
            .save(concat!(stringify!($preset), ".png"))?;

        std::process::Command::new("feh")
            .args(&["-F", concat!(stringify!($preset), ".png")])
            .status()?;
    }};
}
