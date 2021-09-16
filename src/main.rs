mod config;
mod hsv;
mod print;
mod rgb;

use crate::config::{get_args, Config};
use crate::hsv::ColorHSV;
use crate::print::{print, print_rgb};

#[macro_use]
extern crate clap;

fn main() {
    let config = get_args();

    run(config);
}

fn run(config: Config) {
    if config.rainbow {
        print_rgb(config);
    } else {
        print(config);
    }
}
