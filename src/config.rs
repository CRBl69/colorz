use crate::{ColorHSV, rgb::ColorRGB};
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

    let rgb_mode = matches.is_present("rgb");
    let background = matches.is_present("background");
    let rainbow = matches.is_present("rainbow");

    let hsv = if rgb_mode {
        let rgb = matches.value_of("rgb").unwrap().to_string();
        let rgb: Vec<char> = rgb.chars().collect();

        if rgb.len() != 7 {
            eprintln!(
                "[{}]: rgb format but be \"#123abc\", got \"{}\"",
                "ERROR".color(Red),
                matches.value_of("rgb").unwrap()
            );
            std::process::exit(1);
        }

        let r_1  = hex_to_dec(rgb.get(1).unwrap());
        let r_2  = hex_to_dec(rgb.get(2).unwrap());
        let g_1  = hex_to_dec(rgb.get(3).unwrap());
        let g_2  = hex_to_dec(rgb.get(4).unwrap());
        let b_1  = hex_to_dec(rgb.get(5).unwrap());
        let b_2  = hex_to_dec(rgb.get(6).unwrap());

        let red = r_1 * 16 + r_2;
        let green = g_1 * 16 + g_2;
        let blue = b_1 * 16 + b_2;

        ColorRGB::new(red, green, blue).as_hsv()
    } else {
        ColorHSV::new(hue, saturation, value)
    };

    Config {
        file,
        text,
        hsv,
        radius,
        background,
        rainbow,
    }
}

fn hex_to_dec(charachter: &char) -> u8{
    match charachter {
        'f' => 15,
        'e' => 14,
        'd' => 13,
        'c' => 12,
        'b' => 11,
        'a' => 10,
        _ => charachter.to_string().parse::<u8>().unwrap()
    }
}