use crate::ColorHSV;
use regex::Regex;
use std::env;

/// Gets and returns the command line arguments
pub fn get_args() -> (String, ColorHSV, f32, bool) {
    let args: Vec<String> = env::args().collect();

    let text_regex = Regex::new("--text=(.*)").unwrap();
    let hue_regex = Regex::new("--hue=(.*)").unwrap();
    let val_regex = Regex::new("--value=(.*)").unwrap();
    let sat_regex = Regex::new("--saturation=(.*)").unwrap();
    let radius_regex = Regex::new("--radius=(.*)").unwrap();
    let background_regex = Regex::new("--bg").unwrap();

    let mut text = String::new();

    let mut hsv_color = ColorHSV::new(0.0, 1.0, 1.0);

    let mut radius = 1.0;

    let mut bg = false;

    for arg in args.iter() {
        if let Some(txt) = text_regex.captures_iter(arg).next() {
            text = txt[1].to_string();
        }
        if let Some(txt) = hue_regex.captures_iter(arg).next() {
            hsv_color.hue = txt[1].parse::<f32>().unwrap();
        }
        if let Some(txt) = val_regex.captures_iter(arg).next() {
            hsv_color.value = txt[1].parse::<f32>().unwrap();
        }
        if let Some(txt) = sat_regex.captures_iter(arg).next() {
            hsv_color.saturation = txt[1].parse::<f32>().unwrap();
        }
        if let Some(txt) = radius_regex.captures_iter(arg).next() {
            radius = txt[1].parse::<f32>().unwrap();
        }
        if background_regex.captures_iter(arg).next().is_some() {
            bg = true;
        }
    }
    (text, hsv_color, radius, bg)
}