use crate::ColorHSV;
use clap::App;

/// Gets and returns the command line arguments
pub fn get_args() -> (String, String, ColorHSV, f32, bool) {
    let yaml = load_yaml!("clap.yml");

    let matches = App::from_yaml(yaml).get_matches();

    let file = matches.value_of("FILE").unwrap_or("").to_string();

    let text = matches.value_of("text").unwrap_or("").to_string();
    let hue = matches.value_of("hue").unwrap_or("0").parse::<f32>().unwrap();
    let value = matches.value_of("value").unwrap_or("1").parse::<f32>().unwrap();
    let saturation = matches.value_of("saturation").unwrap_or("1").parse::<f32>().unwrap();
    let radius = matches.value_of("radius").unwrap_or("1").parse::<f32>().unwrap();
    let backgroud = matches!(matches.occurrences_of("bg"), 1);

    let hsv_color = ColorHSV::new(hue, saturation, value);

    (file, text, hsv_color, radius, backgroud)
}