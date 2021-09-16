use crate::ColorHSV;
use clap::App;
use colored::{Color::Red, Colorize};

pub struct Config {
    pub file: String,
    pub text: String,
    pub hsv: ColorHSV,
    pub radius: f32,
    pub background: bool,
    pub rainbow: bool,
}

/// Gets and returns the command line arguments
pub fn get_args() -> Config {
    let yaml = load_yaml!("clap.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let file = matches.value_of("FILE").unwrap_or("").to_string();

    if !file.is_empty() && std::fs::metadata(&file).is_err() {
        eprintln!(
            "[{}]: Given file ({}) doesn't exist or you do not have the permissions to access this file",
            "ERROR".color(Red),
            file
        );
        std::process::exit(1);
    }

    let text = matches.value_of("text").unwrap_or("").to_string();
    let hue = matches
        .value_of("hue")
        .unwrap_or("0")
        .parse::<f32>()
        .unwrap_or_else(|_| {
            eprintln!(
                "[{}]: hue must be a float, got \"{}\"",
                "ERROR".color(Red),
                matches.value_of("hue").unwrap()
            );
            std::process::exit(1);
        });
    let value = matches
        .value_of("value")
        .unwrap_or("1")
        .parse::<f32>()
        .unwrap_or_else(|_| {
            eprintln!(
                "[{}]: value must be a float, got \"{}\"",
                "ERROR".color(Red),
                matches.value_of("value").unwrap()
            );
            std::process::exit(1);
        });
    let saturation = matches
        .value_of("saturation")
        .unwrap_or("1")
        .parse::<f32>()
        .unwrap_or_else(|_| {
            eprintln!(
                "[{}]: saturation must be a float, got \"{}\"",
                "ERROR".color(Red),
                matches.value_of("saturation").unwrap()
            );
            std::process::exit(1);
        });
    let radius = matches
        .value_of("radius")
        .unwrap_or("1")
        .parse::<f32>()
        .unwrap_or_else(|_| {
            eprintln!(
                "[{}]: radius must be a float, got \"{}\"",
                "ERROR".color(Red),
                matches.value_of("radius").unwrap()
            );
            std::process::exit(1);
        });

    let background = matches.is_present("background");
    let rainbow = matches.is_present("rainbow");

    let hsv = ColorHSV::new(hue, saturation, value);

    Config {
        file,
        text,
        hsv,
        radius,
        background,
        rainbow,
    }
}
