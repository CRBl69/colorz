use colored::Colorize;
use crate::hsv::ColorHSV;
use std::io::{self, BufRead};

/// Prints a char in the given hsv color
pub fn print_hsv_char(c: char, color: &ColorHSV) {
    let rgb_color = color.to_rgb();
    print!("{}", c.to_string().truecolor(rgb_color.red, rgb_color.green, rgb_color.blue));
}

/// Prints a char on the given hsv color in black or white
/// (depending on the background, optimized for maximum readability)
pub fn print_hsv_char_background(c: char, color: &ColorHSV) {
    let rgb_color = color.to_rgb();
    let color2 = ColorHSV {
        hue: color.hue,
        saturation: 0.0,
        value: if color.value > 0.5 {
            0.0
        } else {
            1.0
        },
    };
    let color_fg = color2.to_rgb();
    print!(
        "{}",
        c.to_string().truecolor(color_fg.red, color_fg.green, color_fg.blue).on_truecolor(
            rgb_color.red,
            rgb_color.green,
            rgb_color.blue
        )
    );
}

pub fn print_rgb(text: &str, mut hsv_color: ColorHSV, radius: f32, on_bg: bool, stdin: io::Stdin) {
    let hue_reserve = hsv_color.hue;

    let mut max = 0;

    let mut lines = Vec::<String>::new();

    if text.is_empty() {
        for line in stdin.lock().lines() {
            lines.push(line.unwrap());
            if lines[lines.len() - 1].chars().count() > max {
                max = lines[lines.len() - 1].chars().count();
            }
        }
    } else {
        let (lines_tmp, max_tmp) = text_to_vec(&text);
        lines = lines_tmp;
        max = max_tmp;
    }

    for line in lines {
        for c in line.chars() {
            if !on_bg {
                print_hsv_char(c, &hsv_color);
            } else {
                print_hsv_char_background(c, &hsv_color);
            }

            hsv_color.hue += (360 / max) as f32 * radius;
            if hsv_color.hue > 360.0 {
                hsv_color.hue -= 360.0;
            }
        }
        println!();
        hsv_color.hue = hue_reserve;
    }
}

/// Transforms a given string into a Vector that represents the lines of the text
pub fn text_to_vec(s: &str) -> (Vec<String>, usize) {
    let mut first_index = 0;
    let mut lines = Vec::<String>::new();
    let mut max = 0;
    for (index, character) in s.chars().enumerate() {
        if character == '\\' && s.as_bytes()[index + 1] == b'n' {
            lines.push(String::from(&s[first_index..index]));
            if index - first_index > max {
                max = index - first_index;
            }
            first_index = index + 2;
        }
    }
    lines.push(String::from(&s[first_index..s.len()]));
    if s.len() - first_index > max {
        max = s.len() - first_index;
    }
    (lines, max)
}