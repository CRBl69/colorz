use colored::Colorize;
use std::env;
use std::io::{self, BufRead};
use regex::Regex;

/// A struct that represents an RGB color
struct ColorRGB {
    red: u8,
    green: u8,
    blue: u8,
}

/// A struct that represents an HSV color
struct ColorHSV {
    hue: f32,
    saturation: f32,
    value: f32,
}

fn main() {
    let stdin = io::stdin();

    let (text, mut hsv_color, radius, on_bg) = get_args();

    let hue_reserve = hsv_color.hue;

    let mut max = 0;

    let mut lines = Vec::<String>::new();

    if text == "" {
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

    for line in 0..lines.len() {
        for c in 0..lines[line].len() {
            if !on_bg {
                print_hsv_char(get_char(&lines[line], c), &hsv_color);
            } else {
                print_hsv_char_background(get_char(&lines[line], c), &hsv_color);
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

/// Gets the char at the given index or returns a ' ' (space) character
/// # Example
/// ```
/// let s = "Hello world !";
/// 
/// assert_eq!(get_char(s, 2), 'l');
/// assert_eq!(get_char(s, 30), ' ');
/// ```
fn get_char(s: &str, index: usize) -> char {
    s.as_bytes()[index] as char
}

/// Prints a char in the given hsv color
fn print_hsv_char(c: char, color: &ColorHSV) {
    let rgb_color = color.to_rgb();
    print!("{}", c.to_string().truecolor(rgb_color.red, rgb_color.green, rgb_color.blue));
}

/// Prints a char on the given hsv color in black or white
/// (depending on the background, optimized for maximum readability)
fn print_hsv_char_background(c: char, color: &ColorHSV) {
    let rgb_color = color.to_rgb();
    let color2 = ColorHSV {
        hue: color.hue,
        saturation: 0.0,
        value: if color.value > 0.5 {0.0} else {1.0},
    };
    let color_bg = color2.to_rgb();
    print!("{}", c.to_string().truecolor(color_bg.red, color_bg.green, color_bg.blue).on_truecolor(rgb_color.red, rgb_color.green, rgb_color.blue));
}

/// Gets and returns the command line arguments
fn get_args() -> (String, ColorHSV, f32, bool) {
    let args: Vec<String> = env::args().collect();

    let text_regex = Regex::new("--text=(.*)").unwrap();
    let hue_regex = Regex::new("--hue=(.*)").unwrap();
    let val_regex = Regex::new("--value=(.*)").unwrap();
    let sat_regex = Regex::new("--saturation=(.*)").unwrap();
    let radius_regex = Regex::new("--radius=(.*)").unwrap();
    let background_regex = Regex::new("--bg=(.*)").unwrap();

    let mut text = String::new();

    let mut hsv_color = ColorHSV::new(0.0, 1.0, 1.0);

    let mut radius = 1.0;

    let mut bg = false;

    for arg in args.iter() {
        for txt in text_regex.captures_iter(arg) {
            text = txt[1].to_string();
            continue;
        }
        for txt in hue_regex.captures_iter(arg) {
            hsv_color.hue = txt[1].parse::<f32>().unwrap();
            continue;
        }
        for txt in val_regex.captures_iter(arg) {
            hsv_color.value = txt[1].parse::<f32>().unwrap();
            continue;
        }
        for txt in sat_regex.captures_iter(arg) {
            hsv_color.saturation = txt[1].parse::<f32>().unwrap();
            continue;
        }
        for txt in radius_regex.captures_iter(arg) {
            radius = txt[1].parse::<f32>().unwrap();
            continue;
        }
        for txt in background_regex.captures_iter(arg) {
            bg = txt[1].parse::<bool>().unwrap();
            continue;
        }
    }
    (text, hsv_color, radius, bg)
}

/// Transforms a given string into a Vector that represents the lines of the text
fn text_to_vec(s: &str) -> (Vec<String>, usize) {
    let mut first_index = 0;
    let mut lines = Vec::<String>::new();
    let mut max = 0;
    for i in 0..s.len() {
        if get_char(s, i) == '\\' && get_char(s, i + 1) == 'n' {
            lines.push(String::from(&s[first_index..i]));
            if i - first_index > max {
                max = i - first_index;
            }
            first_index = i + 2;
        }
    }
    lines.push(String::from(&s[first_index..s.len()]));
    if s.len() - first_index > max {
        max = s.len() - first_index;
    }
    (lines, max)
}

impl ColorHSV {
    /// Converts an hsv color to an rgb color
    /// # Example
    /// ```
    /// let hsv = ColorHSV::new(0.0, 0.0, 0.0);
    ///
    /// let color = ColorRBG::new(0, 0, 0);
    ///
    /// let rgb = hsv.to_rgb();
    ///
    /// assert_eq!(color, rgb);
    /// ```
    fn to_rgb(&self) -> ColorRGB {
        let c = self.value * self.saturation;

        let m: f32 = self.value - c;

        let x: f32 = (c as f32 * (1.0 - ((self.hue / 60 as f32) % 2 as f32 - 1.0).abs())) as f32;

        let (x, y, z) = (
            ((c + m) * 255.0) as u8,
            ((x + m) * 255.0) as u8,
            ((0.0 + m) * 255.0) as u8
        );

        match (self.hue as i32 / 60) as u8 {
            1 => ColorRGB::new(y, x, z),
            2 => ColorRGB::new(z, x, y),
            3 => ColorRGB::new(z, y, x),
            4 => ColorRGB::new(y, z, x),
            5 => ColorRGB::new(x, z, y),
            _ => ColorRGB::new(x, y, z),
        }
    }

    /// Returns a new ColorHSV struct with the given values
    /// # Example
    /// ```
    /// let color = ColorHSV {
    ///     hue: 0.0,
    ///     saturation: 0.0,
    ///     value: 0.0,
    /// }
    /// 
    /// let hsv = ColorHSV::new(0.0, 0.0, 0.0);
    /// 
    /// assert_eq!(color, hsv);
    /// ```
    fn new(hue: f32, saturation: f32, value: f32) -> ColorHSV {
        ColorHSV { hue, saturation, value }
    }
}

impl ColorRGB {
    /// Returns a new ColorRGB struct with the given values
    /// # Example
    /// ```
    /// let color = ColorRGB {
    ///     red: 0,
    ///     green: 0,
    ///     blue: 0,
    /// }
    /// 
    /// let rgb = ColorRGB::new(0, 0, 0);
    /// 
    /// assert_eq!(color, rgb);
    /// ```
    fn new(red: u8, green: u8, blue: u8) -> ColorRGB {
        ColorRGB { red, green, blue }
    }
}
