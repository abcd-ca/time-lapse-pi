use crate::args::TimeLapseConfig;
use crate::gpio;
use crate::number_util::{get_frequency_ms, get_total_images, parse_duration_to_ms};
use std::error::Error;
use std::fs;
use std::process::Command;
use std::time::Duration;
use tokio::time;

// ref: `libcamera-jpeg -h` and https://www.raspberrypi.com/documentation/accessories/camera.html#libcamera-jpeg
fn capture_image(n: u64) -> Result<(), Box<dyn Error + Send + Sync>> {
    // taking stills in a rust loop instead of using the built-in libcamera timelapse so that we can provide GPIO feedback and do disk storage checks
    let output = Command::new("libcamera-jpeg")
        .arg("-o")
        .arg(format!(
            "output/image{:0pad_width$}.jpg",
            n + 1,
            pad_width = 4
        ))
        .arg("--immediate")
        .arg("--width")
        .arg("1280")
        .arg("--height")
        .arg("720")
        .arg("--quality")
        .arg("85")
        // flip because in its enclosure it's upside down
        .arg("--vflip")
        .arg("--nopreview")
        .output()?;

    // TODO estimate disk space needed and warn
    // TODO check disk space between pictures and quit before running out.

    if !output.status.success() {
        // error_chain::bail!("Command executed with failing error code");
        println!("Something went wrong, exiting");

        String::from_utf8(output.stderr)?
            .lines()
            // .filter_map(|line| pattern.captures(line))
            // .map(|cap| Commit {
            //     hash: cap[1].to_string(),
            //     message: cap[2].trim().to_string(),
            // })
            .take(5)
            .for_each(|x| eprintln!("{:?}", x));

        return Err("libcamera-jpeg CLI command failed".into());
        // std::process::exit(1)
    }

    String::from_utf8(output.stdout)?
        .lines()
        // .filter_map(|line| pattern.captures(line))
        // .map(|cap| Commit {
        //     hash: cap[1].to_string(),
        //     message: cap[2].trim().to_string(),
        // })
        .take(5)
        .for_each(|x| println!("{:?}", x));

    Ok(())
}

type TimeLapseResult = Result<(), &'static str>;

pub async fn start_time_lapse(config: &TimeLapseConfig) -> TimeLapseResult {
    let capture_duration_ms = parse_duration_to_ms(&config.capture_duration)?;
    let delay_start_duration_ms = parse_duration_to_ms(&config.delay_start_duration)?;

    let mut x: u64 = 0;

    let total_images = get_total_images(capture_duration_ms, &config.preset);
    let frequency_ms = get_frequency_ms(&config.preset);

    let (mut light, mut button) = gpio::get_peripherals();
    light.on();
    button.wait_for_press("Press the button to start the time-lapse");
    light.off();

    if delay_start_duration_ms > 0 {
        println!(
            "Delaying start for {} milliseconds",
            delay_start_duration_ms
        );
        time::sleep(Duration::from_millis(delay_start_duration_ms)).await;
    }

    println!(
        "Will capture {} images for the time lapse for {}...",
        total_images, config.capture_duration
    );

    fs::create_dir_all("./output").expect("Should create the output directory");

    loop {
        // blink before each capture so you can tell the time-lapse is still recording
        light.blink();
        match capture_image(x) {
            Ok(()) => {
                if x < total_images - 1 {
                    x += 1;
                    println!("Captured {} of {}", x, total_images);
                    time::sleep(Duration::from_millis(frequency_ms)).await;
                } else {
                    println!("Finished. {} images captured.", total_images);
                    // turn LED on so that I can tell when the timelapse is on
                    light.on();
                    button.wait_for_press("Press the button to turn off the LED and exit");
                    light.off();
                    return Ok(());
                }
            }
            Err(_) => return Err("Encountered error, exiting"),
        }
    }
}
//
// // TODO unit test
// fn get_exposure(config: &TimeLapseConfig) -> u32 {
//     unimplemented!("return the aperature exposure based on the --preset and --trails options")
// }
//
