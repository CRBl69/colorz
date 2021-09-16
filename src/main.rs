use std::io;

mod config;
mod print;
mod hsv;
mod rgb;

use crate::hsv::ColorHSV;
use crate::config::get_args;
use crate::print::print_rgb;

fn main() {
    let stdin = io::stdin();

    let (text, hsv_color, radius, on_bg) = get_args();

    print_rgb(&text, hsv_color, radius, on_bg, stdin);
}
