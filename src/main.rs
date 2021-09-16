mod config;
mod print;
mod hsv;
mod rgb;

use crate::hsv::ColorHSV;
use crate::config::get_args;
use crate::print::print_rgb;

#[macro_use]
extern crate clap;

fn main() {
    let (file, text, hsv_color, radius, on_bg) = get_args();

    print_rgb(file, text, hsv_color, radius, on_bg);
}
