use crate::{config::Config, hsv::ColorHSV};
use colored::Colorize;
use std::io::{self, BufRead};

/// Prints a char in the given hsv color
fn print_hsv_char(c: char, color: &ColorHSV) {
    let rgb_color = color.as_rgb();
    print!(
        "{}",
        c.to_string()
            .truecolor(rgb_color.red, rgb_color.green, rgb_color.blue)
    );
}

/// Prints a char on the given hsv color in black or white
/// (depending on the background, optimized for maximum readability)
fn print_hsv_char_background(c: char, color: &ColorHSV) {
    let rgb_color = color.as_rgb();
    let color2 = ColorHSV {
        hue: color.hue,
        saturation: 0.0,
        value: if color.value > 0.5 { 0.0 } else { 1.0 },
    };
    let color_fg = color2.as_rgb();
    print!(
        "{}",
        c.to_string()
            .truecolor(color_fg.red, color_fg.green, color_fg.blue)
            .on_truecolor(rgb_color.red, rgb_color.green, rgb_color.blue)
    );
}

pub fn print(config: Config) {
    let text = if !config.file.is_empty() {
        String::from_utf8(std::fs::read(config.file).expect("The file does not exist")).unwrap()
    } else {
        config.text
    };

    let color_rgb = config.hsv.as_rgb();

    if text.is_empty() {
        for line in std::io::stdin().lock().lines() {
            println!(
                "{}",
                line.unwrap().truecolor(color_rgb.red, color_rgb.green, color_rgb.blue)
            );
        }
    } else {
        println!(
            "{}",
            text.truecolor(color_rgb.red, color_rgb.green, color_rgb.blue)
        );
    }
}

pub fn print_rgb(config: Config) {
    let text = if !config.file.is_empty() {
        String::from_utf8(std::fs::read(config.file).expect("The file does not exist")).unwrap()
    } else {
        config.text
    };

    let mut max = 0;

    let mut lines = Vec::<&str>::new();

    let string_lines: Vec<String>;

    if text.is_empty() {
        let stdin = io::stdin();
        string_lines = stdin.lock().lines().map(|line| line.unwrap()).collect();
        for line in &string_lines {
            lines.push(line);
            if lines[lines.len() - 1].chars().count() > max {
                max = lines[lines.len() - 1].chars().count();
            }
        }
    } else {
        max = get_max_line_length(&text);
        lines = text.lines().collect();
    }

    let mut hsv_color = config.hsv;

    for line in lines {
        for c in line.chars() {
            if !config.background {
                print_hsv_char(c, &hsv_color);
            } else {
                print_hsv_char_background(c, &hsv_color);
            }

            hsv_color.hue += (360 / max) as f32 * config.radius;
            if hsv_color.hue > 360.0 {
                hsv_color.hue -= 360.0;
            }
        }
        println!();
        hsv_color.hue = config.hsv.hue;
    }
}

fn get_max_line_length(s: &str) -> usize {
    let mut max = 0;
    for line in s.lines() {
        if line.len() > max {
            max = line.len();
        }
    }
    max
}
