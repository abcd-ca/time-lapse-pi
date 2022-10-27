extern crate core;

use time_lapse_pi::gpio;

use std::env;

fn exit_with_usage() {
    eprintln!("Usage: `led on` or `led off`");
    std::process::exit(1)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        exit_with_usage();
    }

    let (light, _button) = gpio::get_peripherals();
    let state = &args[1];

    match state.as_str() {
        "on" => light.on(),
        "off" => light.off(),
        _ => exit_with_usage(),
    }
}
