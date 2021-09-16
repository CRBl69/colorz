use crate::ColorHSV;
use clap::App;

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

    let text = matches.value_of("text").unwrap_or("").to_string();
    let hue = matches
        .value_of("hue")
        .unwrap_or("0")
        .parse::<f32>()
        .unwrap();
    let value = matches
        .value_of("value")
        .unwrap_or("1")
        .parse::<f32>()
        .unwrap();
    let saturation = matches
        .value_of("saturation")
        .unwrap_or("1")
        .parse::<f32>()
        .unwrap();
    let radius = matches
        .value_of("radius")
        .unwrap_or("1")
        .parse::<f32>()
        .unwrap();

    let background = matches!(matches.occurrences_of("bg"), 1);
    let rainbow = matches!(matches.occurrences_of("rainbow"), 1);

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
